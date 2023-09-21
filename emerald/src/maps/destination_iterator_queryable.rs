use crate::types::{LinkAndDestination, ResourceId};

/// This trait is used to query all links and their resource id destination.
pub trait DestinationIteratorQueryable {
    fn query_destination_iter(
        &self,
        origin: ResourceId,
    ) -> Option<std::vec::IntoIter<LinkAndDestination>>;
}
