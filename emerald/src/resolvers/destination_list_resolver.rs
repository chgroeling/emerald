use crate::types::{LinkAndResourceId, ResourceId};

/// This trait is used to resolve all links with their desitnation which are orignating from a note
/// given by `resource_id`
pub trait DestinationListResolver {
    fn resolve(&self, resource_id: ResourceId) -> Option<std::vec::IntoIter<LinkAndResourceId>>;
}
