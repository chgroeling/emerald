use crate::types;

#[derive(Clone)]
pub struct NoteMetaData {
    pub title: String,
    pub location: String,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
}

impl From<types::MetaData> for NoteMetaData {
    fn from(value: types::MetaData) -> Self {
        Self {
            title: value.name,
            location: value.location,
            size: value.size,
            modified: value.modified,
            created: value.created,
        }
    }
}