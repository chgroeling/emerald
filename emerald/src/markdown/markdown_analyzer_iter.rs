use super::markdown_iterator_state::{ActionResult, State, StateData, Yield};
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

enum ConsumeResult {
    Some((usize, char)),
    None,
    Eof,
}

/// Represents the result of a character consumption operation in a character iterator.
///
/// # Variants
///
/// - `Some((usize, char))`: Indicates that a character was successfully consumed from the iterator.
///   It holds a tuple containing the index of the character in the original string and the character itself.
///   This variant is typically returned when the character meets the criteria specified in the c
///   onsumption operation.
///
/// - `None`: Represents the scenario where the current character in the iterator does not meet the
///   consumption criteria, and therefore, is not consumed. This variant allows the iterator to remain at
///   the current character, which can then be re-evaluated or consumed by a subsequent operation.
///
/// - `Eof`: Signifies that the end of the input has been reached. This variant is returned when there are
///   no more characters to consume, indicating that the parsing process has reached the end of the input string.
impl ConsumeResult {
    /// Checks if the result contains a index and a character.
    #[allow(dead_code)]
    fn is_some(&self) -> bool {
        matches!(self, ConsumeResult::Some(_))
    }

    /// Checks if the result is `None`.
    #[allow(dead_code)]
    fn is_none(&self) -> bool {
        matches!(self, ConsumeResult::None)
    }

    /// Checks if the result indicates the end of the input (`Eof`).
    #[allow(dead_code)]
    fn is_eof(&self) -> bool {
        matches!(self, ConsumeResult::Eof)
    }

    /// Checks if the result is either `None` or `Eof`.
    #[allow(dead_code)]
    fn is_none_or_eof(&self) -> bool {
        matches!(self, ConsumeResult::None | ConsumeResult::Eof)
    }
}

/// Consumes the next character from the iterator.
///
/// This macro advances the given character iterator and consumes the next character.
/// If there is a next character, it returns a `ConsumeResult::Some`, containing the index
/// and character. If the iterator is at the end of the input, it returns `ConsumeResult::Eof`.
///
/// # Parameters
/// - `$iter`: A mutable reference to a `Peekable<CharIndices>` iterator over a string slice.
///   The iterator must be able to yield pairs of index and character (`(usize, char)`).
///
/// # Returns
/// - `ConsumeResult::Some((usize, char))` if there is a next character in the iterator.
/// - `ConsumeResult::Eof` if the iterator has reached the end of the input string.
macro_rules! consume {
    ($iter:expr) => {
        if let Some((idx, ch)) = $iter.next() {
            ConsumeResult::Some((idx, ch))
        } else {
            ConsumeResult::Eof
        }
    };
}

/// Consumes the next character in the iterator if it matches any of the provided patterns.
///
/// This macro peeks at the next character in the given character iterator. If the character
/// matches any of the specified patterns, the macro consumes this character (advances the iterator)
/// and returns a `ConsumeResult::Some` containing the index and character. If the character does not
/// match any of the patterns, it returns `ConsumeResult::None` without advancing the iterator.
/// If the iterator is at the end, it returns `ConsumeResult::Eof`.
///
/// # Parameters
/// - `$iter`: A mutable reference to a `Peekable<CharIndices>` iterator over a string slice.
/// - `$($a:pat)+`: One or more patterns to match against the next character. These can be
///   simple characters, ranges, or any pattern that can be used in a `match` arm.
///
/// # Returns
/// - `ConsumeResult::Some((usize, char))` if the next character matches any of the provided patterns.
/// - `ConsumeResult::None` if the next character does not match any of the patterns.
/// - `ConsumeResult::Eof` if the iterator has reached the end of the input string.
macro_rules! consume_expected_chars{
    ($iter:expr, $($a:pat)+) => {
        if let Some((index,ch)) = $iter.peek().cloned() {
            match ch {
                $($a)|+ => {
                    $iter.next(); // consume
                    ConsumeResult::Some((index, ch))
                }
                _ => {
                    ConsumeResult::None
                }
            }
        } else {
            ConsumeResult::Eof
        }
    };
}

/// Collects and counts characters from an iterator that match a given pattern.
///
/// This macro iterates over characters from a given character iterator,
/// counting the number of characters that match the specified pattern(s).
/// The iteration stops either when a non-matching character is found or
/// when the specified limit is reached, if provided.
///
/// # Arguments
/// * `$iter` - An expression yielding a `Peekable<CharIndices>` iterator.
/// * `$limit` - An optional limit for the number of characters to count.
///             If `None`, the macro counts until a non-matching character is encountered.
/// * `$($a:pat)+` - One or more patterns to match against each character.
///
/// # Returns
/// The count of matching characters found up to the limit, or until a non-matching character.
/// ```
macro_rules! gather {
    ($iter:expr, $limit: expr, $($a:pat)+) => {{
        // special case .. limit is zero or negative .. do nothing
        if $limit.is_some_and(|limit| limit <= 0) {
            0
        } else {
            let mut cnt: i32 = 0;
            loop {
                if consume_expected_chars!($iter, $($a)|+).is_some() {
                    cnt += 1;

                    if $limit.is_some_and(|l| cnt >= l) {
                        break;
                    }
                } else {
                    break;
                }
            }
            cnt
        }
    }};
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
            let Some((index, i)) = self.state_data.it.peek().cloned() else {
                break;
            };

            let ar: ActionResult = match self.state_data.state {
                DocumentStartState => {
                    match i {
                        // # Start of parsing
                        '-' => Self::detect_yaml_frontmatter(&mut self.state_data, index),
                        ' ' => Self::detect_inline_code_block(&mut self.state_data, index),
                        '\n' => {
                            consume!(self.state_data.it);
                            ActionResult::NextState(EmptyLineState)
                        }
                        _ => ActionResult::NextState(TextState),
                    }
                }
                EmptyLineState => {
                    match i {
                        // # Empty Line found
                        '\n' => {
                            consume!(self.state_data.it);
                            ActionResult::NextState(EmptyLineState)
                        }
                        ' ' => Self::detect_inline_code_block(&mut self.state_data, index),
                        _ => ActionResult::NextState(TextState),
                    }
                }
                NewLineState => {
                    match i {
                        // # New line found
                        ' ' => Self::detect_empty_line(&mut self.state_data),

                        '\n' => {
                            consume!(self.state_data.it);
                            ActionResult::NextState(EmptyLineState)
                        }
                        _ => ActionResult::NextState(TextState),
                    }
                }

                YamlFrontmatterState => match i {
                    ' ' => Self::detect_inline_code_block(&mut self.state_data, index),
                    _ => ActionResult::NextState(TextState),
                },
                InlCodeBlockState => match i {
                    ' ' => Self::detect_inline_code_block(&mut self.state_data, index),
                    _ => ActionResult::NextState(TextState),
                },
                TextState => {
                    match i {
                        // # Text
                        '[' => Self::detect_link_or_wiki_link(&mut self.state_data, index),
                        '`' => Self::detect_code_block(&mut self.state_data, index),
                        '\n' => {
                            consume!(self.state_data.it);
                            ActionResult::NextState(NewLineState)
                        }

                        _ => {
                            consume!(self.state_data.it);
                            ActionResult::NextState(TextState)
                        }
                    }
                }
            };

            match ar {
                // => todo!(),
                ActionResult::NextState(state) => {
                    self.state_data.state = state;
                }
                ActionResult::YieldState(state, yield_state) => {
                    self.state_data.state = state;
                    return Some(self.convert_yield_res_to_md_block(yield_state));
                }
            }
        }
        None
    }
}
