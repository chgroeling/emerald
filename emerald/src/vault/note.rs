use super::{document_metadata::DocumentMetadata, filesystem_metadata::FilesystemMetadata};
use crate::types;

pub struct Note {
    pub rid: types::ResourceId,
    pub title: String,
    pub yaml: String,
    pub markdown: String,
    pub fs_metadata: FilesystemMetadata,
    pub doc_metadata: DocumentMetadata,
}

impl Note {
    pub fn new(
        rid: types::ResourceId,
        title: String,
        yaml: String,
        md: String,
        fs_metadata: FilesystemMetadata,
        doc_metadata: DocumentMetadata,
    ) -> Self {
        Self {
            rid,
            title,
            yaml,
            markdown: md,
            fs_metadata,
            doc_metadata,
        }
    }
}
