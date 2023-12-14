use crate::{
    markdown::{self, MarkdownAnalyzer},
    types::{self, Content},
};

pub struct MarkdownSplitter();

impl MarkdownSplitter {
    pub fn new() -> Self {
        Self()
    }

    pub fn split_yaml_from_md(&self, content: &Content) -> (String, String) {
        let md_analyzer = markdown::MarkdownAnalyzerImpl::new();
        let mut md_iter = md_analyzer.analyze(&content.0);
        let mut yaml_str = "".to_string();
        let first_element = md_iter.next();
        let mut start_of_markdown = 0;
        if let Some(md) = first_element {
            if let types::MdBlock::YamlFrontmatter(yaml) = md {
                // markdown starts when yaml ends
                start_of_markdown = yaml.len();
                yaml_str = yaml.to_string();
            }
        }

        return (yaml_str, content.0[start_of_markdown..].to_string());
    }
}
