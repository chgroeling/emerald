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
        title: String,
        md: String,
        size: u64,
        created: i64,
        modified: i64,
    ) -> Self {
        Self {
            rid,
            title,
            markdown: md,
            size,
            created,
            modified,
        }
    }
}
