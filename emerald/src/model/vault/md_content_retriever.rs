use super::ex_resource_id::VaultResourceIdTrait;

pub trait MdContentRetriever<T>
where
    T: VaultResourceIdTrait,
{
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `rid`: A reference to a `vault::ExResourceId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, rid: &T) -> &str;
}
