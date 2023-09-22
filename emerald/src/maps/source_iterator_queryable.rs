use crate::types::{LinkFromSource, ResourceId};

/// This trait is used to query an source note and and return all contained links with their target.
pub trait SourceIteratorQueryable {
    fn query(&self, target: ResourceId) -> Option<std::vec::IntoIter<LinkFromSource>>;
}
