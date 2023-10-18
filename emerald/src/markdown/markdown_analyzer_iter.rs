use super::markdown_iterator_state::MarkdownIteratorState;
use crate::types::ContentType;
use std::{iter::Peekable, str::CharIndices};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct MarkdownAnalyzerIter<'a> {
    md_str: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> MarkdownAnalyzerIter<'a> {
    pub fn new(md_str: &'a str) -> Self {
        Self {
            md_str,
            iter: md_str.char_indices().peekable(),
        }
    }

    fn expect_then_consume(&mut self, expected_char: char) -> bool {
        let next_element = self.iter.peek();

        if let Some(current_char) = next_element {
            if current_char.1 == expected_char {
                self.iter.next(); // consume
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
            let peek_element = self.iter.peek();
            if let Some((_, ch)) = peek_element {
                if ch == &consume_char {
                    cnt += 1;
                    self.iter.next();

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
        use MarkdownIteratorState::*;

        let open_cnt = 1 + self.consume_char(' ', None);

        if open_cnt < 4 {
            return IllegalFormat;
        }

        let mut next_element = self.iter.next();
        if next_element.is_none() {
            // special case... inline code block at end of file
            return InlCodeBlockFound(start_idx, start_idx + open_cnt as usize);
        }

        // Opening Inline Code Block was detected
        let mut iter_state = InlCodeBlockStart(start_idx);
        let mut act_idx = 0usize;
        while let Some((idx, _)) = next_element {
            let i_peek_opt = self.iter.peek();

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
            next_element = self.iter.next();
        }
        InlCodeBlockFound(start_idx, act_idx + 1)
    }
    fn sm_inline_code_block_after_nl(&mut self, start_idx: usize) -> MarkdownIteratorState {
        use MarkdownIteratorState::*;
        // Is that an inline code block?
        if self.expect_then_consume(' ') {
            self.sm_inline_code_block(start_idx + 1)
        } else {
            IllegalFormat
        }
    }

    fn sm_code_block(&mut self, start_idx: usize) -> MarkdownIteratorState {
        use MarkdownIteratorState::*;

        let open_cnt = 1 + self.consume_char('`', None);

        let mut next_element = self.iter.next();
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
            next_element = self.iter.next()
        }
        IllegalFormat
    }

    fn sm_wiki_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        use MarkdownIteratorState::*;

        let mut next_element = self.iter.next();
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
            next_element = self.iter.next()
        }
        IllegalFormat
    }

    fn sm_link(&mut self, start_idx: usize) -> MarkdownIteratorState {
        use MarkdownIteratorState::*;
        let mut next_element = self.iter.next();

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

            next_element = self.iter.next();
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
    type Item = ContentType<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use ContentType::*;
        use MarkdownIteratorState::*;

        let mut next_element = self.iter.next();

        while let Some((idx, i)) = next_element {
            let iter_state = match i {
                '[' => self.sm_wiki_link_or_link(idx),
                '`' => self.sm_code_block(idx),
                '\n' => self.sm_inline_code_block_after_nl(idx),
                ' ' if idx == 0 => self.sm_inline_code_block(idx),
                _ => IllegalFormat,
            };

            match iter_state {
                WikiLinkFound(s1, e1) => return Some(WikiLink(&self.md_str[s1..e1])),
                LinkFound(s1, e1) => return Some(Link(&self.md_str[s1..e1])),
                CodeBlockFound(s1, e1) => return Some(CodeBlock(&self.md_str[s1..e1])),
                InlCodeBlockFound(s1, e1) => return Some(CodeBlock(&self.md_str[s1..e1])),
                // this also matches IllegalFormat
                _ => next_element = self.iter.next(),
            };
        }

        None
    }
}

// ---------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{ContentType, MarkdownAnalyzerIter};
    use ContentType::*;

    #[test]
    fn test_content_iter_empty_string_empty() {
        let test_str = "";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert!(out_vec.is_empty());
    }

    #[test]
    fn test_content_iter_string_without_links_empty() {
        let test_str = "no links";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert!(out_vec.is_empty());
    }

    #[test]
    fn test_content_iter_simple_wiki_link() {
        let test_str = "[[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_simple_link() {
        let test_str = "[link_name](link)";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [Link("[link_name](link)".into())]);
    }

    #[test]
    fn test_content_iter_two_wiki_links_consecutive() {
        let test_str = "[[internal_link]][[internal_link_2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[internal_link]]".into()),
                WikiLink("[[internal_link_2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_two_wiki_links_consecutive_first_illegal() {
        let test_str = "[[illegal_internal_link] ][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_markdown_link_iter_iter_two_links_consecutive_first_illegal_2() {
        let test_str = "[ [illegal_internal_link]][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_two_links_consecutive_first_illegal_3() {
        let test_str = "[[illegal_internal_link][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_two_links_consecutive_first_illegal_4() {
        let test_str = "[illegal_internal_link]][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_two_links_consecutive_first_illegal_5() {
        let test_str = "[[illegal[_internal_link]][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_two_links_consecutive_first_illegal_6() {
        let test_str = "[[illegal]_internal_link]][[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_two_links_with_distance() {
        let test_str = "[[internal_link]]abc[[internal_link_2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[internal_link]]".into()),
                WikiLink("[[internal_link_2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_two_links_with_distance_start() {
        let test_str = "123[[internal_link]]abc[[internal_link_2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[internal_link]]".into()),
                WikiLink("[[internal_link_2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_two_links_with_distance_start_and_end() {
        let test_str = "123[[internal_link]]abc[[internal_link_2]]456";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[internal_link]]".into()),
                WikiLink("[[internal_link_2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_simple_front_text() {
        let test_str = "abc[[internal_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[internal_link]]".into())]);
    }

    #[test]
    fn test_content_iter_no_link_code_block() {
        let test_str = "abc`[[internal_link]]`";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("`[[internal_link]]`".into())]);
    }

    #[test]
    fn test_content_iter_no_link_code_block_2() {
        let test_str = "abc``[[internal_link]]``";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("``[[internal_link]]``".into())]);
    }

    #[test]
    fn test_content_iter_no_link_code_block_3() {
        let test_str = "abc[[link]]``[[no_link]]``";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[link]]".into()),
                CodeBlock("``[[no_link]]``".into())
            ]
        );
    }

    #[test]
    fn test_markdown_link_iter_no_link_code_block_4() {
        let test_str = "``[[no_link]]``abc[[link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                CodeBlock("``[[no_link]]``".into()),
                WikiLink("[[link]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_no_link_code_block_with_newlines() {
        let test_str = "[[link1]]\n```[[no_link]]\n```\n[[link2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[link1]]".into()),
                CodeBlock("```[[no_link]]\n```".into()),
                WikiLink("[[link2]]".into())
            ]
        );
    }
    #[test]
    fn test_content_iter_no_link_code_block_at_top_with_newlines_and_text() {
        let test_str = "```[[no_link]]\n```\ndef\n[[link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                CodeBlock("```[[no_link]]\n```".into()),
                WikiLink("[[link]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_no_link_code_block_at_end_with_newlines_and_text() {
        let test_str = "def\n[[link]]\n```[[no_link]]\n```\n";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[link]]".into()),
                CodeBlock("```[[no_link]]\n```".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_no_link_code_block_with_newlines_and_text() {
        let test_str = "[[link1]]\nabc\n```[[no_link]]\n```\ndef\n[[link2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[link1]]".into()),
                CodeBlock("```[[no_link]]\n```".into()),
                WikiLink("[[link2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_link_surrounded_by_code_blocks() {
        let test_str = "``code_block``[[link]]``code_block``";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                CodeBlock("``code_block``".into()),
                WikiLink("[[link]]".into()),
                CodeBlock("``code_block``".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_two_links_surrounded_by_code_blocks() {
        let test_str = "``code_block``[[link1]][[link2]]``code_block``";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                CodeBlock("``code_block``".into()),
                WikiLink("[[link1]]".into()),
                WikiLink("[[link2]]".into()),
                CodeBlock("``code_block``".into())
            ]
        );
    }
    #[test]
    fn test_content_iter_no_link_code_block_with_newlines_and_text_and_special_chars() {
        let test_str = "[[link1]]\n—abc—\n```[[—no_link—]]\n```\n—def—\n[[link2]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                WikiLink("[[link1]]".into()),
                CodeBlock("```[[—no_link—]]\n```".into()),
                WikiLink("[[link2]]".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_code_block_in_code_block() {
        let test_str = "```` ```[[no_link]]``` ````";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("```` ```[[no_link]]``` ````".into())]);
    }

    #[test]
    fn test_content_iter_inline_codeblock_first_line() {
        let test_str = "    [[no_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("    [[no_link]]".into()),]);
    }

    #[test]
    fn test_content_iter_inline_codeblock_first_line_with_newline() {
        let test_str = "    [[no_link]]\nText";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("    [[no_link]]".into()),]);
    }

    #[test]
    fn test_content_iter_inline_codeblock_second_line() {
        let test_str = "Text\n    [[no_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("    [[no_link]]".into()),]);
    }

    #[test]
    fn test_content_iter_inline_codeblock_second_line_with_newline() {
        let test_str = "Text\n    [[no_link]]\nText2";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [CodeBlock("    [[no_link]]".into()),]);
    }

    #[test]
    fn test_content_iter_inline_code_blocks() {
        let test_str = "    line1\n    line2\n";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [CodeBlock("    line1".into()), CodeBlock("    line2".into())]
        );
    }

    #[test]
    fn test_content_iter_inline_code_blocks_last_empty() {
        let test_str = "    line1\n    ";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [CodeBlock("    line1".into()), CodeBlock("    ".into())]
        );
    }

    #[test]
    fn test_content_iter_code_block_inside_inline_code_block() {
        let test_str = "    ```line1\n    line2```\n";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(
            out_vec,
            [
                CodeBlock("    ```line1".into()),
                CodeBlock("    line2```".into())
            ]
        );
    }

    #[test]
    fn test_content_iter_link_with_leadinger_underscore() {
        let test_str = "[[_link]]";
        let output = MarkdownAnalyzerIter::new(&test_str);
        let out_vec: Vec<_> = output.collect();

        assert_eq!(out_vec, [WikiLink("[[_link]]".into())]);
    }
}
