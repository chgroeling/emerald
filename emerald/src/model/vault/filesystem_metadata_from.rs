use super::{filesystem_metadata::FilesystemMetadata, timestamp::Timestamp};
use crate::model::note::NoteMetadata;

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
