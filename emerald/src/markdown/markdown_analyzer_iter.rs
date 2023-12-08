use super::markdown_iterator_state::MarkdownIteratorState;
use crate::types;
use std::{iter::Peekable, str::CharIndices};
use MarkdownIteratorState::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct MarkdownAnalyzerIter<'a> {
    buf: &'a str,
    it: Peekable<CharIndices<'a>>,
}

/// Checks and consumes the next char in the iterator if it matches the provided pattern(s).
///
/// - `$iter`: The iterator containing the input sequence.
/// - `$($a:pat)+`: Pattern(s) to match against the next char.
/// If the next char matches, it's consumed and returned as `Some(char)`. Otherwise, returns `None`.
macro_rules! consume_expected_chars{
    ($iter:expr, $($a:pat)+) => {
        if let Some((_,ch)) = $iter.peek().cloned() {
            match ch {
                $($a)|+ => {
                    $iter.next(); // consume
                    Some(ch)
                }
                _ => {
                    None
                }
            }
        } else {
            None
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
        }
    }

    fn detect_inline_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let open_cnt = 1 + gather!(self.it, Option::<i32>::None, ' ');

        if open_cnt < 4 {
            return IllegalFormat;
        }

        let mut next_element = self.it.next();
        if next_element.is_none() {
            // special case... inline code block at end of file
            return InlCodeBlockFound(start_idx, start_idx + open_cnt as usize);
        }

        // Opening Inline Code Block was detected
        let mut iter_state = InlCodeBlockStart(start_idx);
        let mut act_idx = 0usize;
        while let Some((idx, _)) = next_element {
            let i_peek_opt = self.it.peek();

            if let Some((idx_peek, i_peek)) = i_peek_opt {
                // determine new state
                iter_state = match iter_state {
                    InlCodeBlockStart(start_idx) if i_peek == &'\n' => {
                        return InlCodeBlockFound(start_idx, *idx_peek);
                    }
                    _ => iter_state,
                };
            }
            act_idx = idx;

            // action for new state
            next_element = self.it.next();
        }
        InlCodeBlockFound(start_idx, act_idx + 1)
    }

    fn detect_inline_code_block_after_newline(
        &mut self,
        start_idx: usize,
    ) -> MarkdownIteratorState {
        // Is that an inline code block?

        if consume_expected_chars!(self.it, ' ').is_some() {
            self.detect_inline_code_block(start_idx + 1)
        } else {
            IllegalFormat
        }
    }

    fn detect_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let open_cnt = 1 + gather!(self.it, Option::<i32>::None, '`');
        let mut next_element = self.it.next();
        if next_element.is_none() {
            return IllegalFormat;
        }

        // Opening code block was detected
        let mut iter_state = CodeBlockStart(start_idx);

        while let Some((idx, i)) = next_element {
            // determine new state
            iter_state = match iter_state {
                CodeBlockStart(start_idx) if i == '`' => {
                    let advance = 1 + gather!(self.it, Option::<i32>::Some(open_cnt - 1), '`');

                    if advance == open_cnt {
                        let end_idx = idx + 1 + advance as usize - 1;

                        return CodeBlockFound(start_idx, end_idx);
                    } else {
                        assert!(advance < open_cnt);
                        iter_state
                    }
                }
                _ => iter_state,
            };

            // action for new state
            next_element = self.it.next()
        }
        IllegalFormat
    }

    fn detect_wiki_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let mut next_element = self.it.next();
        if next_element.is_none() {
            return IllegalFormat;
        }

        // Opening of an internal link was detected
        let mut iter_state = WikiLinkStart(start_idx);

        while let Some((idx, i)) = next_element {
            // determine new state
            iter_state = match iter_state {
                WikiLinkStart(start_idx) if i == ']' => {
                    // Match ]] ...
                    if consume_expected_chars!(self.it, ']').is_some() {
                        return WikiLinkFound(start_idx, idx + 2);
                    } else {
                        return IllegalFormat;
                    }
                }
                WikiLinkStart(_) if i == '[' => return IllegalFormat,
                _ => iter_state,
            };

            // action for new state
            next_element = self.it.next()
        }
        IllegalFormat
    }

    fn detect_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let mut next_element = self.it.next();

        // Opening of an internal link was detected
        let mut iter_state = LinkStart(start_idx);

        while let Some((idx, i)) = next_element {
            iter_state = match iter_state {
                LinkStart(start_idx) if i == ']' => {
                    // next char must be '('
                    if consume_expected_chars!(self.it, '(').is_some() {
                        LinkDescriptionFinished(start_idx)
                    } else {
                        return IllegalFormat;
                    }
                }
                LinkDescriptionFinished(start_idx) if i == ')' => {
                    return LinkFound(start_idx, idx + 1)
                }

                _ => iter_state,
            };

            next_element = self.it.next();
        }

        IllegalFormat
    }

    fn detect_link_or_wiki_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        if consume_expected_chars!(self.it, '[').is_some() {
            // wiki link starts with '[['
            self.detect_wiki_link(start_idx)
        } else {
            // conventional link starts with '['
            self.detect_link(start_idx)
        }
    }
}

impl<'a> Iterator for MarkdownAnalyzerIter<'a> {
    type Item = types::MdBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, i)) = self.it.next() {
            // Determine the markdown element based on the current character
            let markdown_element = match i {
                '[' => self.detect_link_or_wiki_link(index),
                '`' => self.detect_code_block(index),
                '\n' => self.detect_inline_code_block_after_newline(index),
                ' ' if index == 0 => self.detect_inline_code_block(index),
                _ => IllegalFormat,
            };

            use types::MdBlock as ct; // short hand for the following code
            match markdown_element {
                WikiLinkFound(s1, e1) => return Some(ct::WikiLink(&self.buf[s1..e1])),
                LinkFound(s1, e1) => return Some(ct::Link(&self.buf[s1..e1])),
                CodeBlockFound(s1, e1) => return Some(ct::CodeBlock(&self.buf[s1..e1])),
                InlCodeBlockFound(s1, e1) => return Some(ct::CodeBlock(&self.buf[s1..e1])),
                IllegalFormat => (), // Skip illegal formats and continue searching
                _ => panic!("State is not expected here"),
            };
        }
        None
    }
}
