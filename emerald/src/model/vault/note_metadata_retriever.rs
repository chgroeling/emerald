use super::{
    ex_resource_id::VaultResourceIdTrait, DocumentMetadata, FilesystemMetadata, VaultResourceId,
};

/// Trait for retrieving metadata associated with a note.
pub trait NoteMetadataRetriever<T>
where
    T: VaultResourceIdTrait,
{
    /// Retrieves metadata for a given note.
    ///
    /// # Arguments
    ///
    /// * `tgt` - Target note's resource identifier.
    ///
    /// # Returns
    ///
    /// A tuple containing the note's title (`String`), `FilesystemMetadata`,
    /// and `DocumentMetadata`.
    fn retrieve(&self, tgt: &VaultResourceId<T>) -> (String, FilesystemMetadata, DocumentMetadata);
}
