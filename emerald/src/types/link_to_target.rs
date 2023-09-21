use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a Link and its destination Resource Id if existant.
pub struct LinkToTarget {
    pub link: Link,
    pub target: Option<ResourceId>,
}

impl LinkToTarget {
    pub fn new(link: Link, target: Option<ResourceId>) -> Self {
        Self { link, target }
    }
    pub fn new_without_target(link: Link) -> Self {
        Self { link, target: None }
    }
    pub fn new_with_target(link: Link, target_resource_id: ResourceId) -> Self {
        Self {
            link,
            target: Some(target_resource_id),
        }
    }
}
