use super::{DocumentMetadata, ExResourceId, FilesystemMetadata};

/// Trait for retrieving metadata associated with a note.
pub trait NoteMetadataRetriever {
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
    fn retrieve(&self, tgt: &ExResourceId) -> (String, FilesystemMetadata, DocumentMetadata);
}
