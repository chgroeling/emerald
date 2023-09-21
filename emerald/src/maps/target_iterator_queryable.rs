use crate::types::{LinkToTarget, ResourceId};

/// This trait is used to query an source note and and return all contained links with their target.
pub trait TargetIteratorQueryable {
    fn query(&self, source: ResourceId) -> Option<std::vec::IntoIter<LinkToTarget>>;
}
