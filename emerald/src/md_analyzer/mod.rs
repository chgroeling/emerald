use self::markdown_analyzer_iter::MarkdownAnalyzerIter;
use crate::types::ContentType;

mod markdown_analyzer_iter;
mod markdown_iterator_state;

pub fn analyze_markdown(md_str: &str) -> impl Iterator<Item = ContentType> {
    MarkdownAnalyzerIter::new(md_str)
}

struct AnalyzeMarkdownImpl;
trait AnalyzeMarkdown<'a> {
    type Iterator: Iterator<Item = ContentType<'a>> + 'a;
    fn analyze(md_str: &'a str) -> Self::Iterator;
}

impl<'a> AnalyzeMarkdown<'a> for AnalyzeMarkdownImpl {
    type Iterator = MarkdownAnalyzerIter<'a>;

    fn analyze(md_str: &'a str) -> Self::Iterator {
        MarkdownAnalyzerIter::new(md_str)
    }
}
