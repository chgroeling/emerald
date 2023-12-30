use super::{DocumentMetadata, FilesystemMetadata, VaultResourceId};

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait NoteMetadataRetriever {
    fn retrieve(&self, tgt: &VaultResourceId) -> (String, FilesystemMetadata, DocumentMetadata);
}
