use super::{Link, ResourceId};

#[derive(Debug, Clone, PartialEq)]
// Structs holds a link and the resource id of the links target if existant.
pub struct Link2Tgt {
    pub link: Link,
    pub tgt: Option<ResourceId>,
}

impl Link2Tgt {
    pub fn new(link: Link, tgt: Option<ResourceId>) -> Self {
        Self { link, tgt }
    }
    /*
    pub fn new_without_target(link: Link) -> Self {
        Self { link, tgt: None }
    }
    pub fn new_with_target(link: Link, tgt_rid: ResourceId) -> Self {
        Self {
            link,
            tgt: Some(tgt_rid),
        }
    }
    */
}
