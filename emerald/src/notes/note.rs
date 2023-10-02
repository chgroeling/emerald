use crate::notes::providers::{md_provider::MdProvider, title_provider::TitleProvider};

use crate::types::ResourceId;

pub struct Note {
    resource_id: ResourceId,
    title_provider: Box<dyn TitleProvider>,
    markdown_provider: Box<dyn MdProvider>,
}

impl Note {
    pub fn new(
        resource_id: ResourceId,
        title_provider: Box<dyn TitleProvider>,
        markdown_provider: Box<dyn MdProvider>,
    ) -> Self {
        Self {
            resource_id,
            title_provider,
            markdown_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get_title(&self.resource_id)
    }

    pub fn markdown(&self) -> String {
        self.markdown_provider.get_markdown(&self.resource_id)
    }
}
