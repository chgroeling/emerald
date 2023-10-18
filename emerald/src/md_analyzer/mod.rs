use self::markdown_analyzer_iter::MarkdownAnalyzerIter;
use crate::types::ContentType;

mod markdown_analyzer_iter;
mod markdown_iterator_state;

pub fn analyze_markdown(md_str: &str) -> impl Iterator<Item = ContentType> {
    MarkdownAnalyzerIter::new(md_str)
}
