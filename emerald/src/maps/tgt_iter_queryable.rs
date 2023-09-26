use crate::types::{LinkToTarget, ResourceId};

/// This trait is used to query an source id and and return all links which points to this source
pub trait TgtIterQueryable {
    fn query(&self, src: ResourceId) -> Option<std::vec::IntoIter<LinkToTarget>>;
}
