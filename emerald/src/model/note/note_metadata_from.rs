use super::note_metadata::{DocumentMetadata, FilesystemMetadata, NoteMetadata};
use crate::types;

impl From<(types::FilesystemMetadata, types::DocumentMetadata)> for NoteMetadata {
    fn from(value: (types::FilesystemMetadata, types::DocumentMetadata)) -> Self {
        let fs_md = FilesystemMetadata {
            location: value.0.location,
            size: value.0.size,
            modified: value.0.modified,
            created: value.0.created,
            ..Default::default()
        };

        let doc_md = DocumentMetadata {
            aliases: value.1.aliases.unwrap_or(vec![]),
            keywords: value.1.keywords.unwrap_or(vec![]),
            modified: value.1.modified,
            created: value.1.created,
            tags: value.1.tags,
            ..Default::default()
        };

        Self {
            title: value.0.name,
            filesystem: fs_md,
            document: doc_md,
            ..Default::default()
        }
    }
}