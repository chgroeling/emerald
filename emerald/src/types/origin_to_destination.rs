use super::{LinkAndResourceId, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds a links with is destination (the place where it points to) and the
/// Resource Id of the origin, the place where it stems from
#[allow(dead_code)]
pub struct OriginToDestination {
    pub origin: ResourceId,
    pub link_and_destination: LinkAndResourceId,
}
