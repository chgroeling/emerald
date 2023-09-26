use super::{Link, Link2Tgt, LinkFrmSrc, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds the source of a link and its target (the place where it points to).
/// Source and target are concrete ResourceIds which are pointing to files.
#[allow(dead_code)]
pub struct LinkSrc2Tgt {
    pub src: ResourceId,
    pub link: Link,
    pub tgt: Option<ResourceId>,
}

impl LinkSrc2Tgt {
    pub fn new(src: ResourceId, link: Link, tgt: Option<ResourceId>) -> Self {
        Self { src, link, tgt }
    }

    pub fn from_link_to_target(src: ResourceId, link_2_tgt: Link2Tgt) -> Self {
        Self::new(src, link_2_tgt.link, link_2_tgt.tgt)
    }

    pub fn get_link_to_target(&self) -> Link2Tgt {
        Link2Tgt::new(self.link.clone(), self.tgt.clone())
    }

    pub fn get_link_from_source(&self) -> LinkFrmSrc {
        LinkFrmSrc::new(self.link.clone(), self.src.clone())
    }
}

/// Allows to generate LinkFrmSrcToTarget from string tuple
/// (source, link, target)
impl From<(&str, &str, &str)> for LinkSrc2Tgt {
    fn from(value: (&str, &str, &str)) -> Self {
        LinkSrc2Tgt::new(value.0.into(), value.1.into(), Some(value.2.into()))
    }
}
