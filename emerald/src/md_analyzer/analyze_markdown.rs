use crate::types::ContentType;

use super::markdown_analyzer_iter::MarkdownAnalyzerIter;

pub fn analyze_markdown<'a>(md_str: &'a String) -> impl Iterator<Item = ContentType> + 'a {
    MarkdownAnalyzerIter::new(md_str)
}
