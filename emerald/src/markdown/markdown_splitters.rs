/// Module contains markdown splitters.
///
/// This module provides a struct, `MarkdownFrontMatterSplitter`, that can be used to split the YAML
/// frontmatter from the markdown content.
///
use super::markdown_analyzer::MarkdownAnalyzer;
use super::markdown_analyzer_impl::MarkdownAnalyzerImpl;
use crate::types;

/// A trait representing a MarkdownFrontMatterSplitter
pub trait MarkdownFrontmatterSplitter {
    /// Splits the YAML frontmatter from the markdown content.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content.
    ///
    /// # Returns
    ///
    /// A tuple containing the YAML frontmatter and the remaining markdown content.
    fn split<'a>(&self, content: &'a str) -> (&'a str, &'a str) {
        let md_analyzer = MarkdownAnalyzerImpl::new();
        let mut md_iter = md_analyzer.analyze(content);
        let mut yaml_str = "";
        let first_element = md_iter.next();
        let mut start_of_markdown = 0;
        if let Some(types::MdBlock::YamlFrontmatter(yaml)) = first_element {
            // markdown starts when yaml ends
            start_of_markdown = yaml.len();
            yaml_str = yaml;
        }

        (yaml_str, &content[start_of_markdown..])
    }
    fn trim_pre_and_postamble<'a>(&self, content: &'a str) -> &'a str {
        content
            .trim_start_matches("---")
            .trim_start_matches('\n')
            .trim_end_matches('\n')
            .trim_end_matches("---")
            .trim_end_matches('\n')
    }
}
/// A struct representing a MarkdownFrontMatterSplitterImpl.
#[derive(Clone)]
pub struct MarkdownFrontmatterSplitterImpl();
impl Copy for MarkdownFrontmatterSplitterImpl {}
impl MarkdownFrontmatterSplitterImpl {
    /// Creates a new instance of MarkdownSplitter.
    ///
    /// # Returns
    ///
    /// A new instance of MarkdownSplitter.
    pub fn new() -> Self {
        Self()
    }
}
impl MarkdownFrontmatterSplitter for MarkdownFrontmatterSplitterImpl {}
