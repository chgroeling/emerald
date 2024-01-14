use super::{link_query_result::LinkQueryResult, VaultResourceId};

/// Trait for querying links contained in a target resource.
pub trait GetLinks<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    /// Returns an iterator over links contained in the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_links_of(
        &self,
        rid: &VaultResourceId<T>,
    ) -> Box<dyn Iterator<Item = LinkQueryResult<T>>>;
}
