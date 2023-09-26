use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a link and the resource id from which the links stems.
pub struct LinkFrmSrc {
    pub link: Link,
    pub src: ResourceId,
}

impl LinkFrmSrc {
    pub fn new(link: Link, src: ResourceId) -> Self {
        Self { src, link }
    }
}
