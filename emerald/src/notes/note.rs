use crate::notes::providers::{content_provider::ContentProvider, title_provider::TitleProvider};

use crate::types::ResourceId;

pub struct Note {
    resource_id: ResourceId,
    title_provider: Box<dyn TitleProvider>,
    content_provider: Box<dyn ContentProvider>,
}

impl Note {
    pub fn new(
        resource_id: ResourceId,
        title_provider: Box<dyn TitleProvider>,
        content_provider: Box<dyn ContentProvider>,
    ) -> Self {
        Self {
            resource_id,
            title_provider,
            content_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get_title(&self.resource_id)
    }

    pub fn content(&self) -> String {
        self.content_provider.get_content(&self.resource_id)
    }
}
