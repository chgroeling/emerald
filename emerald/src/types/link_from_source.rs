use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a link and the resource id of the links target if existant.
pub struct LinkFromSource {
    pub source: ResourceId,
    pub link: Link,
}

impl LinkFromSource {
    pub fn new(source: ResourceId, link: Link) -> Self {
        Self { source, link }
    }
}
