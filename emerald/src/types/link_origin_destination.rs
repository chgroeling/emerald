use super::{Link, ResourceId};

#[derive(Debug, Clone)]
/// This struct hold a link, the origin where the links stems from and
/// and its destination were it points to if it exists..
#[allow(dead_code)]
pub struct LinkOriginDestination {
    pub link: Link,
    pub origin: ResourceId,
    pub destination: Option<ResourceId>,
}
