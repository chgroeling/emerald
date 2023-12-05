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

impl<'a> MarkdownAnalyzerIter<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            it: buf.char_indices().peekable(),
        }
    }

    fn expect_then_consume(&mut self, expected_char: char) -> bool {
        let next_element = self.it.peek();

        if let Some(next_char) = next_element {
            if next_char.1 == expected_char {
                self.it.next(); // consume
                return true;
            }
        }
        false
    }

    fn consume_char(&mut self, consume_char: char, limit: Option<i32>) -> i32 {
        let mut cnt: i32 = 0;

        // special case .. limit is zero or negative .. do nothing
        if limit.is_some_and(|limit| limit <= 0) {
            return 0;
        }

        loop {
            let peek_element = self.it.peek();
            if let Some((_, ch)) = peek_element {
                if ch == &consume_char {
                    cnt += 1;
                    self.it.next();

                    if limit.is_some_and(|l| cnt >= l) {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        cnt
    }

    fn sm_inline_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let open_cnt = 1 + self.consume_char(' ', None);

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

    fn sm_inline_code_block_after_nl(&mut self, start_idx: usize) -> MarkdownIteratorState {
        // Is that an inline code block?
        if self.expect_then_consume(' ') {
            self.sm_inline_code_block(start_idx + 1)
        } else {
            IllegalFormat
        }
    }

    fn sm_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let open_cnt = 1 + self.consume_char('`', None);
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
                    let advance = 1 + self.consume_char('`', Some(open_cnt - 1));

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

    fn sm_wiki_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
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
                    if self.expect_then_consume(']') {
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

    fn sm_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        let mut next_element = self.it.next();

        // Opening of an internal link was detected
        let mut iter_state = LinkStart(start_idx);

        while let Some((idx, i)) = next_element {
            iter_state = match iter_state {
                LinkStart(start_idx) if i == ']' => {
                    if self.expect_then_consume('(') {
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

    fn sm_wiki_link_or_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        if self.expect_then_consume('[') {
            // wiki link starts with '[['
            self.sm_wiki_link(start_idx)
        } else {
            // conventional link starts with '['
            self.sm_link(start_idx)
        }
    }
}

impl<'a> Iterator for MarkdownAnalyzerIter<'a> {
    type Item = types::MdBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_element = self.it.next();
        while let Some((idx, i)) = next_element {
            let iter_state = match i {
                '[' => self.sm_wiki_link_or_link(idx),
                '`' => self.sm_code_block(idx),
                '\n' => self.sm_inline_code_block_after_nl(idx),
                ' ' if idx == 0 => self.sm_inline_code_block(idx),
                _ => IllegalFormat,
            };

            use types::MdBlock as ct; // short hand for the following code
            match iter_state {
                WikiLinkFound(s1, e1) => return Some(ct::WikiLink(&self.buf[s1..e1])),
                LinkFound(s1, e1) => return Some(ct::Link(&self.buf[s1..e1])),
                CodeBlockFound(s1, e1) => return Some(ct::CodeBlock(&self.buf[s1..e1])),
                InlCodeBlockFound(s1, e1) => return Some(ct::CodeBlock(&self.buf[s1..e1])),
                IllegalFormat => next_element = self.it.next(), // search for next valid md
                _ => panic!("State is not expected here"),
            };
        }
        None
    }
}
