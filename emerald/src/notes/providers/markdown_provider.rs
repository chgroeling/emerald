use crate::types::ResourceId;

pub trait MarkdownProvider {
    fn get_markdown(&self, resource_id: &ResourceId) -> String;
}
