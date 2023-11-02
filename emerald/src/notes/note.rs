use super::providers::{MdProvider, StringProvider, TimestampProvider};
use crate::types;

pub struct Note {
    rid: types::ResourceId,
    title_provider: Box<dyn StringProvider>,
    md_provider: Box<dyn MdProvider>,
    created_provider: Box<dyn TimestampProvider>,
    modified_provider: Box<dyn TimestampProvider>,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title_provider: Box<dyn StringProvider>,
        md_provider: Box<dyn MdProvider>,
        created_provider: Box<dyn TimestampProvider>,
        modified_provider: Box<dyn TimestampProvider>,
    ) -> Self {
        Self {
            rid,
            title_provider,
            md_provider,
            created_provider,
            modified_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get_string(&self.rid)
    }

    pub fn markdown(&self) -> String {
        self.md_provider.get_markdown(&self.rid)
    }

    pub fn created(&self) -> i64 {
        self.created_provider.get_timestamp(&self.rid)
    }

    pub fn modified(&self) -> i64 {
        self.modified_provider.get_timestamp(&self.rid)
    }
}
