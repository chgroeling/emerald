use super::{resource_id, Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a Link and its destination Resource Id if existant.
pub struct LinkAndDestination {
    pub link: Link,
    pub destination: Option<ResourceId>,
}

impl LinkAndDestination {
    pub fn new(link: Link, destination: Option<ResourceId>) -> Self {
        Self { link, destination }
    }
    pub fn new_without_destination(link: Link) -> Self {
        Self {
            link,
            destination: None,
        }
    }
    pub fn new_with_destination(link: Link, dest_resource_id: ResourceId) -> Self {
        Self {
            link,
            destination: Some(dest_resource_id),
        }
    }
}
