use super::{markdown_analyzer::MarkdownAnalyzer, markdown_analyzer_iter::MarkdownAnalyzerIter};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct MarkdownAnalyzerLocal;

impl MarkdownAnalyzerLocal {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> MarkdownAnalyzer<'a> for MarkdownAnalyzerLocal {
    type Iterator = MarkdownAnalyzerIter<'a>;

    fn analyze(&self, md_str: &'a str) -> Self::Iterator {
        MarkdownAnalyzerIter::new(md_str)
    }
}
