use super::{uid_trait::UidTrait, DocumentMetadata, FilesystemMetadata};

/// Trait for retrieving metadata associated with a note.
pub trait NoteMetadataRetriever<U>
where
    U: UidTrait,
{
    /// Retrieves metadata for a given note.
    ///
    /// # Arguments
    ///
    /// * `tgt` - Target note's unique identifier.
    ///
    /// # Returns
    ///
    /// A tuple containing the note's title (`String`), `FilesystemMetadata`,
    /// and `DocumentMetadata`.
    fn retrieve(&self, tgt: &U) -> (String, FilesystemMetadata, DocumentMetadata);
}
