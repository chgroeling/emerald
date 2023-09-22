use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a link and the resource id from which the links stems.
pub struct LinkFromSource {
    pub link: Link,
    pub source: ResourceId,
}

impl LinkFromSource {
    pub fn new(link: Link, source: ResourceId) -> Self {
        Self { source, link }
    }
}
