use crate::types::{LinkAndResourceId, ResourceId};

pub type ListOfLinksWithDestination = Vec<LinkAndResourceId>;

/// This trait is used to resolve all links with their desitnation which are orignating from a note
/// given by `resource_id`
pub trait DestinationListResolver {
    fn resolve(&self, resource_id: ResourceId) -> std::vec::IntoIter<ListOfLinksWithDestination>;
}
