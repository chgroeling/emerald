use crate::types::{LinkToTarget, ResourceId};

/// This trait is used to query all links and their resource id target.
pub trait DestinationIteratorQueryable {
    fn query_destination_iter(
        &self,
        origin: ResourceId,
    ) -> Option<std::vec::IntoIter<LinkToTarget>>;
}
