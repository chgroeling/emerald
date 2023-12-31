use crate::model::note;
use crate::model::vault;

impl From<&note::NoteMetadata> for vault::FilesystemMetadata {
    fn from(value: &note::NoteMetadata) -> Self {
        Self {
            path: value.filesystem.path.to_owned(),
            size: value.filesystem.size,
            modified: vault::Timestamp(value.filesystem.modified),
            created: vault::Timestamp(value.filesystem.created),
        }
    }
}
