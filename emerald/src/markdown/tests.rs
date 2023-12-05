#[cfg(test)]
mod tests {
    use crate::markdown::markdown_analyzer_iter::MarkdownAnalyzerIter;
    use crate::types::MdBlock::*;

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
