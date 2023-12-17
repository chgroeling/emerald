/// Module contains markdown splitters.
///
/// This module provides a struct, `MarkdownFrontMatterSplitter`, that can be used to split the YAML
/// frontmatter from the markdown content.
///
use super::markdown_analyzer::MarkdownAnalyzer;
use super::markdown_analyzer_impl::MarkdownAnalyzerImpl;
use crate::types;

/// A struct representing a MarkdownFrontMatterSplitter.
pub struct MarkdownFrontMatterSplitter();

impl MarkdownFrontMatterSplitter {
    /// Creates a new instance of MarkdownSplitter.
    ///
    /// # Returns
    ///
    /// A new instance of MarkdownSplitter.
    pub fn new() -> Self {
        Self()
    }

    /// Splits the YAML frontmatter from the markdown content.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content.
    ///
    /// # Returns
    ///
    /// A tuple containing the YAML frontmatter and the remaining markdown content.
    pub fn split(&self, content: &types::Content) -> (String, String) {
        let md_analyzer = MarkdownAnalyzerImpl::new();
        let mut md_iter = md_analyzer.analyze(&content.0);
        let mut yaml_str = "".to_string();
        let first_element = md_iter.next();
        let mut start_of_markdown = 0;
        if let Some(types::MdBlock::YamlFrontmatter(yaml)) = first_element {
            // markdown starts when yaml ends
            start_of_markdown = yaml.len();
            yaml_str = yaml.to_string();
        }

        return (yaml_str, content.0[start_of_markdown..].to_string());
    }
}
