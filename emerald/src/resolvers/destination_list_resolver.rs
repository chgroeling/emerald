use crate::types::{OriginToDestination, ResourceId};

pub type LinkOriginDestinationList = Vec<OriginToDestination>;

/// This trait is used to resolve all links which are orignating from the note
/// given by `resource_id`
pub trait DestinationListResolver {
    fn resolve(&self, resource_id: ResourceId) -> LinkOriginDestinationList;
}
