use super::{link_query_result::LinkQueryResult, vault_resource_id_trait::VaultResourceIdTrait};

/// Trait for querying links contained in a target resource.
pub trait GetLinks<T>
where
    T: VaultResourceIdTrait,
{
    /// Returns an iterator over links contained in the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_links_of(&self, rid: &T) -> Box<dyn Iterator<Item = LinkQueryResult<T>>>;
}
