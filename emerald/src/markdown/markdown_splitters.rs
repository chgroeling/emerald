/// Module contains markdown splitters.
///
/// This module provides a struct, `MarkdownFrontmatterSplitter`, that can be used to split the YAML
/// frontmatter from the markdown content.
///
use super::markdown_analyzer::MarkdownAnalyzer;
use super::markdown_analyzer_impl::MarkdownAnalyzerImpl;
use crate::types;

/// A trait representing a MarkdownFrontMatterSplitter
pub trait MarkdownFrontmatterSplitter {
    /// Splits the YAML frontmatter from the markdown content.
    ///
    /// This method analyzes the given markdown content and attempts to identify and separate
    /// the YAML frontmatter section from the main markdown body. The YAML frontmatter is typically
    /// enclosed within triple-dashed lines (`---`) at the beginning of the markdown file.
    ///
    /// # Arguments
    ///
    /// * `content` - A string slice (`&'a str`) representing the markdown content to be split.
    ///               It should contain the markdown text along with the optional YAML frontmatter.
    ///
    /// # Returns
    ///
    /// A tuple containing two elements:
    /// 1. An `Option<&'a str>` representing the extracted YAML frontmatter, if present.
    ///    It returns `None` if no YAML frontmatter is detected.
    /// 2. A string slice (`&'a str`) representing the remaining markdown content after the
    ///    frontmatter is removed. If no frontmatter is present, this will be the entire input content.
    fn split<'a>(&self, content: &'a str) -> (Option<&'a str>, &'a str);
}
/// A struct representing a MarkdownFrontMatterSplitterImpl.
#[derive(Clone)]
pub struct DefaultMarkdownFrontmatterSplitter();
impl Copy for DefaultMarkdownFrontmatterSplitter {}
impl DefaultMarkdownFrontmatterSplitter {
    /// Creates a new instance of MarkdownSplitter.
    ///
    /// # Returns
    ///
    /// A new instance of MarkdownSplitter.
    pub fn new() -> Self {
        Self()
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
impl MarkdownFrontmatterSplitter for DefaultMarkdownFrontmatterSplitter {
    fn split<'a>(&self, content: &'a str) -> (Option<&'a str>, &'a str) {
        let md_analyzer = MarkdownAnalyzerImpl::new();
        let mut md_iter = md_analyzer.analyze(content);
        let yaml: Option<&str>;
        let first_element = md_iter.next();
        let mut start_of_markdown = 0;
        if let Some(types::MdBlock::YamlFrontmatter(yaml_str)) = first_element {
            // markdown starts when yaml ends
            start_of_markdown = yaml_str.len();
            yaml = Some(self.trim_pre_and_postamble(yaml_str));
        } else {
            yaml = None;
        }

        (yaml, &content[start_of_markdown..])
    }
}
