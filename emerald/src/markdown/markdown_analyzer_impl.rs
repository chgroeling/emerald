use super::markdown_analyzer::MarkdownAnalyzer;
use super::markdown_analyzer_iter::MarkdownAnalyzerIter;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct MarkdownAnalyzerImpl;

impl MarkdownAnalyzerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> MarkdownAnalyzer<'a> for MarkdownAnalyzerImpl {
    type Iterator = MarkdownAnalyzerIter<'a>;

    fn analyze(&self, md_str: &'a str) -> Self::Iterator {
        MarkdownAnalyzerIter::new(md_str)
    }
}
