mod document_metadata;
mod filesystem_metadata;
mod timestamp;

pub use self::document_metadata::DocumentMetadata;
pub use self::filesystem_metadata::FilesystemMetadata;
pub use self::timestamp::Timestamp;
use super::Uid;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct Note {
    pub uid: Uid,
    pub title: String,
    pub yaml: String,
    pub markdown: String,
    pub fs_metadata: FilesystemMetadata,
    pub doc_metadata: DocumentMetadata,
}

impl Note {
    pub fn new(
        uid: Uid,
        title: String,
        yaml: String,
        markdown: String,
        fs_metadata: FilesystemMetadata,
        doc_metadata: DocumentMetadata,
    ) -> Self {
        Self {
            uid,
            title,
            yaml,
            markdown,
            fs_metadata,
            doc_metadata,
        }
    }
}
