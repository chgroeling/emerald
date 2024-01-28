use super::resource_id_trait::ResourceIdTrait;

/// Trait for retrieving metadata associated with a note.
pub trait UidMetadataRetriever<T>
where
    T: ResourceIdTrait,
{
    /// Retrieves uid metadata for a given note.
    ///
    /// # Arguments
    ///
    /// * `tgt` - Target note's resource identifier.
    ///
    /// # Returns
    ///
    /// UID stored in meta data.
    fn retrieve(&self, rid: &T) -> Option<String>;
}
