use super::ResourceId;

use crate::providers::title_provider::TitleProvider;

pub struct Note {
    resource_id: ResourceId,
    title_provider: Box<dyn TitleProvider>,
}

impl Note {
    pub fn new(resource_id: ResourceId, title_provider: Box<dyn TitleProvider>) -> Self {
        Self {
            resource_id,
            title_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get_title(&self.resource_id)
    }
}
