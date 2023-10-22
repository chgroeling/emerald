use super::providers::{MdProvider, TitleProvider};
use crate::types;

pub struct Note {
    resource_id: types::ResourceId,
    title_provider: Box<dyn TitleProvider>,
    md_provider: Box<dyn MdProvider>,
}

impl Note {
    pub fn new(
        resource_id: types::ResourceId,
        title_provider: Box<dyn TitleProvider>,
        md_provider: Box<dyn MdProvider>,
    ) -> Self {
        Self {
            resource_id,
            title_provider,
            md_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get_title(&self.resource_id)
    }

    pub fn markdown(&self) -> String {
        self.md_provider.get_markdown(&self.resource_id)
    }
}
