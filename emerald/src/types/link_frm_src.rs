use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a link and the resource id from which the links stems.
pub struct LinkFrmSrc {
    pub link: Link,
    pub source: ResourceId,
}

impl LinkFrmSrc {
    pub fn new(link: Link, source: ResourceId) -> Self {
        Self { source, link }
    }
}
