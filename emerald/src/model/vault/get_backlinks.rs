use super::{link_query_result::LinkQueryResult, resource_id_trait::ResourceIdTrait};

/// Trait for querying links pointing to a target resource.
pub trait GetBacklinks<T>
where
    T: ResourceIdTrait,
{
    /// Returns an iterator over links pointing to the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_backlinks_of(&self, rid: &T) -> Box<dyn Iterator<Item = LinkQueryResult<T>>>;
}
