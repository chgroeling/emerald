use crate::types;

use super::timestamp::Timestamp;

pub struct Note {
    pub rid: types::ResourceId,
    pub title: String,
    pub markdown: String,
    pub size: u64,
    pub created: Timestamp,
    pub modified: Timestamp,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title: String,
        md: String,
        size: u64,
        created: Timestamp,
        modified: Timestamp,
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
