use super::markdown_iterator_state::{ActionResult, State, StateData, Yield};
use super::utils::*;
use crate::types;
use State::*;
use Yield::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct MarkdownAnalyzerIter<'a> {
    buf: &'a str,
    state_data: StateData<'a>,
}

impl<'a> MarkdownAnalyzerIter<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            state_data: StateData {
                state: DocumentStartState,
                it: buf.char_indices().peekable(),
            },
        }
    }

    fn detect_inline_code_block(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        let open_cnt = gather!(state_data.it, Option::<i32>::None, ' ');

        if open_cnt < 4 {
            return ActionResult::NextState(EmptyLineState);
        }

        // Opening Inline Code Block was detected
        let mut act_idx = start_idx + open_cnt as usize - 1; // -1 because the first char doesn't count

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
                break;
            };

            if i == '\n' {
                return ActionResult::YieldState(InlCodeBlockState, CodeBlock(start_idx, idx));
            }

            act_idx = idx;
        }

        // end of file handling
        ActionResult::YieldState(InlCodeBlockState, CodeBlock(start_idx, act_idx + 1))
    }

    fn detect_code_block(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        if consume_expected_chars!(state_data.it, '`').is_none() {
            return ActionResult::NextState(TextState);
        }

        let open_cnt = 1 + gather!(state_data.it, Option::<i32>::None, '`');

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
                break;
            };

            if i == '`' {
                let advance = 1 + gather!(state_data.it, Option::<i32>::Some(open_cnt - 1), '`');

                if advance == open_cnt {
                    let end_idx = idx + 1 + advance as usize - 1;

                    return ActionResult::YieldState(TextState, CodeBlock(start_idx, end_idx));
                }
            }
        }
        ActionResult::NextState(TextState)
    }

    fn detect_wiki_link(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
                break;
            };

            match i {
                ']' => {
                    // Match ]] ...
                    if consume_expected_chars!(state_data.it, ']').is_some() {
                        return ActionResult::YieldState(TextState, WikiLink(start_idx, idx + 2));
                    } else {
                        return ActionResult::NextState(TextState);
                    }
                }

                '[' => {
                    return ActionResult::NextState(TextState);
                }
                _ => (),
            }
        }
        ActionResult::NextState(TextState)
    }

    fn detect_link(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        enum LinkState {
            LinkStart(usize),
            LinkDescriptionFinished(usize),
        }

        // Opening of an internal link was detected
        let mut link_state: LinkState = LinkState::LinkStart(start_idx);

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(state_data.it) else {
                break;
            };

            link_state = match link_state {
                LinkState::LinkStart(start_idx) if i == ']' => {
                    // next char must be '('
                    if consume_expected_chars!(state_data.it, '(').is_some() {
                        LinkState::LinkDescriptionFinished(start_idx)
                    } else {
                        return ActionResult::NextState(TextState);
                    }
                }
                LinkState::LinkDescriptionFinished(start_idx) if i == ')' => {
                    return ActionResult::YieldState(TextState, Link(start_idx, idx + 1));
                }

                _ => link_state,
            };
        }

        ActionResult::NextState(TextState)
    }

    fn detect_link_or_wiki_link(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        if consume_expected_chars!(state_data.it, '[').is_none() {
            return ActionResult::NextState(TextState);
        }

        if consume_expected_chars!(state_data.it, '[').is_some() {
            // wiki link starts with '[['
            Self::detect_wiki_link(state_data, start_idx)
        } else {
            // conventional link starts with '['
            Self::detect_link(state_data, start_idx)
        }
    }

    /// Detects an empty line in the markdown input.
    ///
    /// This method checks if the current position of the iterator corresponds to
    /// an empty line in the markdown input. An empty line is defined as a sequence of
    /// spaces followed by an optional carriage return ('\r') and a newline character (`\n`).
    /// It consumes all the spaces leading up to the newline character.
    ///
    /// The method is called when parsing markdown to correctly identify and handle
    /// empty lines, which can be significant in markdown syntax, especially in
    /// determining the boundaries of different markdown elements.
    ///
    /// # Returns
    /// * `MarkdownIteratorState::EmptyLineFound` if an empty line is detected.
    /// * `MarkdownIteratorState::IllegalFormat` if the next character is not a newline
    ///   or the end of the iterator is reached, which implies an illegal or unexpected format.
    fn detect_empty_line(state_data: &mut StateData) -> ActionResult {
        // gather all whitespaces doesnt matter how many
        gather!(state_data.it, Option::<i32>::None, ' ');

        // consume optional carriage return
        consume_expected_chars!(state_data.it, '\r');

        // check if the following char is a newline
        if consume_expected_chars!(state_data.it, '\n').is_some() {
            ActionResult::NextState(EmptyLineState)
        } else {
            ActionResult::NextState(TextState)
        }
    }

    /// Detects the presence of YAML front matter in markdown input.
    ///
    /// YAML front matter is typically used at the beginning of markdown documents to contain metadata,
    /// enclosed within triple-dashed lines (`---`). This method identifies the start and end of the
    /// YAML front matter block, if present, and returns a corresponding `MarkdownIteratorState`.
    ///
    /// The detection starts from the current position of the iterator and proceeds until it either confirms
    /// the presence of a valid YAML front matter block or encounters a format that invalidates the block.
    ///
    /// # Parameters
    /// - `start_idx`: The starting index in the markdown text where the detection should begin.
    ///
    /// # Returns
    /// - `MarkdownIteratorState::YamlFrontmatterFound(start_idx, end_idx)` if a YAML front matter block
    ///   is detected, where `start_idx` is the starting index and `end_idx` is the index immediately after
    ///   the end of the block.
    /// - `MarkdownIteratorState::IllegalFormat` if the detected block does not conform to the expected YAML
    ///    front matter format.
    fn detect_yaml_frontmatter(state_data: &mut StateData, start_idx: usize) -> ActionResult {
        // gather 3 dashes
        if gather!(state_data.it, Option::<i32>::None, '-') != 3 {
            return ActionResult::NextState(TextState);
        }
        // consume optional carriage return
        consume_expected_chars!(state_data.it, '\r');

        if consume_expected_chars!(state_data.it, '\n').is_none_or_eof() {
            return ActionResult::NextState(TextState);
        }

        let mut last_index: usize = 0;
        loop {
            let Some((index, i)) = state_data.it.next() else {
                break;
            };

            last_index = index; // needed in case of eof
            if i == '\n' {
                // assume a dash after a newline
                if consume_expected_chars!(state_data.it, '-').is_none_or_eof() {
                    continue;
                }

                last_index += 1;

                // gather 2 more dashes
                let dash_cnt = gather!(state_data.it, Option::<i32>::None, '-');
                if dash_cnt != 2 {
                    last_index += dash_cnt as usize;
                    continue;
                }
                // +1 since the index relates to the newline
                // +3 since 3 dashes were found
                let mut end_index = 1 + index + 3;

                // gather all whitespaces doesnt matter how many
                let ws_count = gather!(state_data.it, Option::<i32>::None, ' ');
                end_index += ws_count as usize;

                // consume optional carriage return
                if consume_expected_chars!(state_data.it, '\r').is_some() {
                    end_index += 1;
                }

                if consume_expected_chars!(state_data.it, '\n').is_some() {
                    end_index += 1usize;
                    return ActionResult::YieldState(
                        YamlFrontmatterState,
                        YamlFrontmatter(start_idx, end_index),
                    );
                }
            }
        }

        ActionResult::YieldState(
            YamlFrontmatterState,
            YamlFrontmatter(start_idx, last_index + 1),
        )
    }

    fn document_start_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };
        match i {
            // # Start of parsing
            '-' => Self::detect_yaml_frontmatter(state_data, index),
            ' ' => Self::detect_inline_code_block(state_data, index),
            '\n' => {
                consume!(state_data.it);
                ActionResult::NextState(EmptyLineState)
            }
            _ => ActionResult::NextState(TextState),
        }
    }

    fn empty_line_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            // # Empty Line found
            '\n' => {
                consume!(state_data.it);
                ActionResult::NextState(EmptyLineState)
            }
            ' ' => Self::detect_inline_code_block(state_data, index),
            _ => ActionResult::NextState(TextState),
        }
    }

    fn new_line_state(state_data: &mut StateData) -> ActionResult {
        let Some((_, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            // # New line found
            ' ' => Self::detect_empty_line(state_data),

            '\n' => {
                consume!(state_data.it);
                ActionResult::NextState(EmptyLineState)
            }
            _ => ActionResult::NextState(TextState),
        }
    }

    fn yaml_frontmatter_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            ' ' => Self::detect_inline_code_block(state_data, index),
            _ => ActionResult::NextState(TextState),
        }
    }

    fn inline_codeblock_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            ' ' => Self::detect_inline_code_block(state_data, index),
            _ => ActionResult::NextState(TextState),
        }
    }

    fn text_state(state_data: &mut StateData) -> ActionResult {
        let Some((index, i)) = state_data.it.peek().cloned() else {
            return ActionResult::EndOfFile;
        };

        match i {
            // # Text
            '[' => Self::detect_link_or_wiki_link(state_data, index),
            '`' => Self::detect_code_block(state_data, index),
            '\n' => {
                consume!(state_data.it);
                ActionResult::NextState(NewLineState)
            }

            _ => {
                consume!(state_data.it);
                ActionResult::NextState(TextState)
            }
        }
    }

    fn convert_yield_res_to_md_block(&self, inp: Yield) -> types::MdBlock<'a> {
        match inp {
            YamlFrontmatter(s, e) => types::MdBlock::YamlFrontmatter(&self.buf[s..e]),
            CodeBlock(s, e) => types::MdBlock::CodeBlock(&self.buf[s..e]),
            WikiLink(s, e) => types::MdBlock::WikiLink(&self.buf[s..e]),
            Link(s, e) => types::MdBlock::Link(&self.buf[s..e]),
        }
    }
}

impl<'a> Iterator for MarkdownAnalyzerIter<'a> {
    type Item = types::MdBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ar: ActionResult = match self.state_data.state {
                DocumentStartState => Self::document_start_state(&mut self.state_data),
                EmptyLineState => Self::empty_line_state(&mut self.state_data),
                NewLineState => Self::new_line_state(&mut self.state_data),
                YamlFrontmatterState => Self::yaml_frontmatter_state(&mut self.state_data),
                InlCodeBlockState => Self::inline_codeblock_state(&mut self.state_data),
                TextState => Self::text_state(&mut self.state_data),
            };

            match ar {
                ActionResult::EndOfFile => {
                    return None;
                }
                ActionResult::NextState(state) => {
                    self.state_data.state = state;
                }
                ActionResult::YieldState(state, yield_state) => {
                    self.state_data.state = state;
                    return Some(self.convert_yield_res_to_md_block(yield_state));
                }
            }
        }
    }
}
