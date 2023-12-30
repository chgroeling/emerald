use super::{link_query_result::LinkQueryResult, ExResourceId};

/// Trait for querying links pointing to a target resource.
pub trait GetBacklinks {
    /// Returns an iterator over links pointing to the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_backlinks_of(&self, rid: &ExResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
