use super::providers::Provider;
use crate::types;

pub struct Note {
    pub rid: types::ResourceId,
    title_provider: Box<dyn Provider<String>>,
    md_provider: Box<dyn Provider<String>>,
    size_provider: u64,
    created_provider: i64,
    modified_provider: i64,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title_provider: Box<dyn Provider<String>>,
        md_provider: Box<dyn Provider<String>>,
        size_provider: u64,
        created_provider: i64,
        modified_provider: i64,
    ) -> Self {
        Self {
            rid,
            title_provider,
            md_provider,
            size_provider,
            created_provider,
            modified_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get(&self.rid)
    }

    pub fn markdown(&self) -> String {
        self.md_provider.get(&self.rid)
    }

    pub fn size(&self) -> u64 {
        self.size_provider
    }

    pub fn created(&self) -> i64 {
        self.created_provider
    }

    pub fn modified(&self) -> i64 {
        self.modified_provider
    }
}
