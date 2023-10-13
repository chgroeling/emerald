use crate::types::ContentType;

use super::markdown_analyzer_iter::MarkdownAnalyzerIter;

pub fn analyze_markdown(md_str: &str) -> impl Iterator<Item = ContentType> {
    MarkdownAnalyzerIter::new(md_str)
}
