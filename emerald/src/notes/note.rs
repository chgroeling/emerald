use crate::types;

pub struct Note {
    pub rid: types::ResourceId,
    pub title: String,
    pub markdown: String,
    pub size: u64,
    pub created: i64,
    pub modified: i64,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title_provider: String,
        md_provider: String,
        size_provider: u64,
        created_provider: i64,
        modified_provider: i64,
    ) -> Self {
        Self {
            rid,
            title: title_provider,
            markdown: md_provider,
            size: size_provider,
            created: created_provider,
            modified: modified_provider,
        }
    }
}
