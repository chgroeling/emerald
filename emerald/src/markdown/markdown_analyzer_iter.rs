use super::markdown_iterator_state::{ActionResult, MarkdownIteratorState};
use crate::types;
use std::{iter::Peekable, str::CharIndices};
use MarkdownIteratorState::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct MarkdownAnalyzerIter<'a> {
    buf: &'a str,
    it: Peekable<CharIndices<'a>>,
    last_state: MarkdownIteratorState,
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
            it: buf.char_indices().peekable(),
            last_state: StartOfParsing,
        }
    }

    fn detect_inline_code_block(&mut self, start_idx: usize) -> ActionResult {
        let open_cnt = gather!(self.it, Option::<i32>::None, ' ');

        if open_cnt < 4 {
            return ActionResult::NextState(EmptyLineFound);
        }

        // Opening Inline Code Block was detected
        let mut act_idx = start_idx + open_cnt as usize - 1; // -1 because the first char doesn't count

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(self.it) else {
                break;
            };

            if i == '\n' {
                return ActionResult::Yield(InlCodeBlockFound(start_idx, idx));
            }

            act_idx = idx;
        }

        // end of file handling
        return ActionResult::Yield(InlCodeBlockFound(start_idx, act_idx + 1));
    }

    fn detect_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        if consume_expected_chars!(self.it, '`').is_none() {
            return IllegalFormat;
        }

        let open_cnt = 1 + gather!(self.it, Option::<i32>::None, '`');

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(self.it) else {
                break;
            };

            if i == '`' {
                let advance = 1 + gather!(self.it, Option::<i32>::Some(open_cnt - 1), '`');

                if advance == open_cnt {
                    let end_idx = idx + 1 + advance as usize - 1;

                    return CodeBlockFound(start_idx, end_idx);
                }
            }
        }
        IllegalFormat
    }

    fn detect_wiki_link(&mut self, start_idx: usize) -> ActionResult {
        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(self.it) else {
                break;
            };

            match i {
                ']' => {
                    // Match ]] ...
                    if consume_expected_chars!(self.it, ']').is_some() {
                        return ActionResult::Yield(WikiLinkFound(start_idx, idx + 2));
                    } else {
                        return ActionResult::NextState(IllegalFormat);
                    }
                }

                '[' => {
                    return ActionResult::NextState(IllegalFormat);
                }
                _ => (),
            }
        }
        ActionResult::NextState(IllegalFormat)
    }

    fn detect_link(&mut self, start_idx: usize) -> ActionResult {
        // Opening of an internal link was detected
        let mut iter_state = LinkStart(start_idx);

        loop {
            // end of file detection
            let ConsumeResult::Some((idx, i)) = consume!(self.it) else {
                break;
            };

            iter_state = match iter_state {
                LinkStart(start_idx) if i == ']' => {
                    // next char must be '('
                    if consume_expected_chars!(self.it, '(').is_some() {
                        LinkDescriptionFinished(start_idx)
                    } else {
                        return ActionResult::NextState(IllegalFormat);
                    }
                }
                LinkDescriptionFinished(start_idx) if i == ')' => {
                    return ActionResult::Yield(LinkFound(start_idx, idx + 1));
                }

                _ => iter_state,
            };
        }

        ActionResult::NextState(IllegalFormat)
    }

    fn detect_link_or_wiki_link(&mut self, start_idx: usize) -> ActionResult {
        if consume_expected_chars!(self.it, '[').is_none() {
            return ActionResult::NextState(IllegalFormat);
        }

        if consume_expected_chars!(self.it, '[').is_some() {
            // wiki link starts with '[['
            self.detect_wiki_link(start_idx)
        } else {
            // conventional link starts with '['
            self.detect_link(start_idx)
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
    fn detect_empty_line(&mut self) -> MarkdownIteratorState {
        // gather all whitespaces doesnt matter how many
        gather!(self.it, Option::<i32>::None, ' ');

        // consume optional carriage return
        consume_expected_chars!(self.it, '\r');

        // check if the following char is a newline
        if consume_expected_chars!(self.it, '\n').is_some() {
            EmptyLineFound
        } else {
            IllegalFormat
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
    fn detect_yaml_frontmatter(&mut self, start_idx: usize) -> ActionResult {
        // gather 3 dashes
        if gather!(self.it, Option::<i32>::None, '-') != 3 {
            return ActionResult::NextState(IllegalFormat);
        }
        // consume optional carriage return
        consume_expected_chars!(self.it, '\r');

        if consume_expected_chars!(self.it, '\n').is_none_or_eof() {
            return ActionResult::NextState(IllegalFormat);
        }

        let mut last_index: usize = 0;
        loop {
            let Some((index, i)) = self.it.next() else {
                break;
            };

            last_index = index; // needed in case of eof
            if i == '\n' {
                // assume a dash after a newline
                if consume_expected_chars!(self.it, '-').is_none_or_eof() {
                    continue;
                }

                last_index += 1;

                // gather 2 more dashes
                let dash_cnt = gather!(self.it, Option::<i32>::None, '-');
                if dash_cnt != 2 {
                    last_index += dash_cnt as usize;
                    continue;
                }
                // +1 since the index relates to the newline
                // +3 since 3 dashes were found
                let mut end_index = 1 + index + 3;

                // gather all whitespaces doesnt matter how many
                let ws_count = gather!(self.it, Option::<i32>::None, ' ');
                end_index += ws_count as usize;

                // consume optional carriage return
                if consume_expected_chars!(self.it, '\r').is_some() {
                    end_index += 1;
                }

                if consume_expected_chars!(self.it, '\n').is_some() {
                    end_index += 1usize;
                    return ActionResult::Yield(YamlFrontmatterFound(start_idx, end_index));
                }
            }
        }

        return ActionResult::Yield(YamlFrontmatterFound(start_idx, last_index + 1));
    }

    fn convert_state_to_md_block(&self, inp: MarkdownIteratorState) -> types::MdBlock<'a> {
        let yield_panic = || panic!("This state has now yield representation");
        match inp {
            YamlFrontmatterFound(s, e) => types::MdBlock::YamlFrontmatter(&self.buf[s..e]),
            InlCodeBlockFound(s, e) => types::MdBlock::CodeBlock(&self.buf[s..e]),
            CodeBlockFound(s, e) => types::MdBlock::CodeBlock(&self.buf[s..e]),
            WikiLinkFound(s, e) => types::MdBlock::WikiLink(&self.buf[s..e]),
            LinkFound(s, e) => types::MdBlock::Link(&self.buf[s..e]),
            IllegalFormat => yield_panic(),
            StartOfParsing => yield_panic(),
            EmptyLineFound => yield_panic(),
            NewLineFound => yield_panic(),
            LinkStart(_) => yield_panic(),
            LinkDescriptionFinished(_) => yield_panic(),
        }
    }
}

impl<'a> Iterator for MarkdownAnalyzerIter<'a> {
    type Item = types::MdBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((index, i)) = self.it.peek().cloned() else {
                break;
            };

            let ar: ActionResult = match self.last_state {
                StartOfParsing => {
                    match i {
                        // # Start of parsing
                        '-' => self.detect_yaml_frontmatter(index),
                        ' ' => self.detect_inline_code_block(index),
                        '\n' => {
                            consume!(self.it);
                            ActionResult::NextState(EmptyLineFound)
                        }
                        _ => ActionResult::NextState(IllegalFormat),
                    }
                }
                EmptyLineFound => {
                    match i {
                        // # Empty Line found
                        '\n' => {
                            consume!(self.it);
                            ActionResult::NextState(EmptyLineFound)
                        }
                        ' ' => self.detect_inline_code_block(index),
                        _ => ActionResult::NextState(IllegalFormat),
                    }
                }
                NewLineFound => {
                    match i {
                        // # New line found
                        ' ' => ActionResult::NextState(self.detect_empty_line()),

                        '\n' => {
                            consume!(self.it);
                            ActionResult::NextState(EmptyLineFound)
                        }
                        _ => ActionResult::NextState(IllegalFormat),
                    }
                }

                YamlFrontmatterFound(_, _) => match i {
                    ' ' => self.detect_inline_code_block(index),
                    _ => ActionResult::NextState(IllegalFormat),
                },
                InlCodeBlockFound(_, _) => match i {
                    ' ' => self.detect_inline_code_block(index),
                    _ => ActionResult::NextState(IllegalFormat),
                },
                _ => {
                    match i {
                        // # Text
                        '[' => self.detect_link_or_wiki_link(index),
                        '`' => {
                            let res = self.detect_code_block(index);

                            // YIELD
                            if let CodeBlockFound(_, _) = res {
                                ActionResult::Yield(res)
                            } else {
                                ActionResult::NextState(res)
                            }
                        }

                        '\n' => {
                            consume!(self.it);
                            ActionResult::NextState(NewLineFound)
                        }

                        _ => {
                            consume!(self.it);
                            ActionResult::NextState(IllegalFormat)
                        }
                    }
                }
            };

            match ar {
                // => todo!(),
                ActionResult::NextState(state) => {
                    self.last_state = state;
                }
                ActionResult::Yield(state) => {
                    self.last_state = state.clone();
                    return Some(self.convert_state_to_md_block(state));
                }
            }
        }
        None
    }
}
