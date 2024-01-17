use super::resource_id_trait::ResourceIdTrait;

pub trait MdContentRetriever<T>
where
    T: ResourceIdTrait,
{
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `rid`: A reference to a `types::ResourceId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, rid: &T) -> &str;
}
