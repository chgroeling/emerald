use super::{link_query_result::LinkQueryResult, vault_resource_id_trait::VaultResourceIdTrait};

/// Trait for querying links pointing to a target resource.
pub trait GetBacklinks<T>
where
    T: VaultResourceIdTrait,
{
    /// Returns an iterator over links pointing to the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_backlinks_of(&self, rid: &T) -> Box<dyn Iterator<Item = LinkQueryResult<T>>>;
}
