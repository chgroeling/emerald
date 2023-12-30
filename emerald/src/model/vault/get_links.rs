use super::{link_query_result::LinkQueryResult, ExResourceId};

/// Trait for querying links contained in a target resource.
pub trait GetLinks {
    /// Returns an iterator over links contained in the specified resource.
    ///
    /// # Arguments
    ///
    /// * `rid`: Resource identifier.
    fn get_links_of(&self, rid: &ExResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
