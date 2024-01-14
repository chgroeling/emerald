use super::{link_query_result::LinkQueryResult, VaultResourceId};

/// Trait for querying links pointing to a target resource.
pub trait GetBacklinks<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    /// Returns an iterator over links pointing to the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_backlinks_of(
        &self,
        rid: &VaultResourceId<T>,
    ) -> Box<dyn Iterator<Item = LinkQueryResult<T>>>;
}
