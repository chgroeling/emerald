mod document_metadata;
mod filesystem_metadata;
mod timestamp;

use crate::types;

pub use self::document_metadata::DocumentMetadata;
pub use self::filesystem_metadata::FilesystemMetadata;
pub use self::timestamp::Timestamp;

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
