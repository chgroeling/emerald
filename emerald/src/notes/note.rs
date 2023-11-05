use super::providers::Provider;
use crate::types;

pub struct Note {
    pub rid: types::ResourceId,
    title_provider: Box<dyn Provider<String>>,
    md_provider: Box<dyn Provider<String>>,
    size_provider: Box<dyn Provider<u64>>,
    created_provider: Box<dyn Provider<i64>>,
    modified_provider: Box<dyn Provider<i64>>,
    linked_notes_provider: Box<dyn Provider<Box<dyn Iterator<Item = Note>>>>,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title_provider: Box<dyn Provider<String>>,
        md_provider: Box<dyn Provider<String>>,
        size_provider: Box<dyn Provider<u64>>,
        created_provider: Box<dyn Provider<i64>>,
        modified_provider: Box<dyn Provider<i64>>,
        linked_notes_provider: Box<dyn Provider<Box<dyn Iterator<Item = Note>>>>,
    ) -> Self {
        Self {
            rid,
            title_provider,
            md_provider,
            size_provider,
            created_provider,
            modified_provider,
            linked_notes_provider,
        }
    }

    pub fn title(&self) -> String {
        self.title_provider.get(&self.rid)
    }

    pub fn markdown(&self) -> String {
        self.md_provider.get(&self.rid)
    }

    pub fn size(&self) -> u64 {
        self.size_provider.get(&self.rid)
    }

    pub fn created(&self) -> i64 {
        self.created_provider.get(&self.rid)
    }

    pub fn modified(&self) -> i64 {
        self.modified_provider.get(&self.rid)
    }

    pub fn linked_notes(&self) -> impl Iterator<Item = Note> {
        self.linked_notes_provider.get(&self.rid).into_iter()
    }
}
