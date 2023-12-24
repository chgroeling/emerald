use super::note;
use super::vault;

impl From<&note::NoteMetadata> for vault::FilesystemMetadata {
    fn from(value: &note::NoteMetadata) -> Self {
        Self {
            location: value.filesystem.location.to_owned(),
            size: value.filesystem.size,
            modified: vault::Timestamp(value.filesystem.modified),
            created: vault::Timestamp(value.filesystem.created),
        }
    }
}
