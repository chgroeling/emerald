use super::content_type::ContentType;
use super::markdown_analyzer_iter::MarkdownAnalyzerIter;

pub fn analyze_markdown(md_str: &String) -> Vec<ContentType> {
    MarkdownAnalyzerIter::new(md_str).collect()
}
