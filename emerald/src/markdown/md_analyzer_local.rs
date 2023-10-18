use super::{markdown_analyzer_iter::MarkdownAnalyzerIter, md_analyzer::MdAnalyzer};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct MdAnalyzerLocal;

impl MdAnalyzerLocal {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> MdAnalyzer<'a> for MdAnalyzerLocal {
    type Iterator = MarkdownAnalyzerIter<'a>;

    fn analyze(&self, md_str: &'a str) -> Self::Iterator {
        MarkdownAnalyzerIter::new(md_str)
    }
}
