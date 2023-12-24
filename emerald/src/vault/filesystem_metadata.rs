use super::timestamp::Timestamp;
use crate::model::note::NoteMetadata;

pub struct FilesystemMetadata {
    pub location: String,
    pub size: u64,
    pub modified: Timestamp,
    pub created: Timestamp,
}

impl From<&NoteMetadata> for FilesystemMetadata {
    fn from(value: &NoteMetadata) -> Self {
        Self {
            location: value.filesystem.location.to_owned(),
            size: value.filesystem.size,
            modified: Timestamp(value.filesystem.modified),
            created: Timestamp(value.filesystem.created),
        }
    }
}
