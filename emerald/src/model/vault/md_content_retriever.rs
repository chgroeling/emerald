use super::vault_resource_id_trait::VaultResourceIdTrait;

pub trait MdContentRetriever<T>
where
    T: VaultResourceIdTrait,
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
