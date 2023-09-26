use super::{Link, Link2Tgt, LinkFrmSrc, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds the source of a link and its target (the place where it points to).
/// Source and target are concrete ResourceIds which are pointing to files.
#[allow(dead_code)]
pub struct LinkSrc2Tgt {
    pub source: ResourceId,
    pub link: Link,
    pub target: Option<ResourceId>,
}

impl LinkSrc2Tgt {
    pub fn new(source: ResourceId, link: Link, target: Option<ResourceId>) -> Self {
        Self {
            source,
            link,
            target,
        }
    }

    pub fn from_link_to_target(source: ResourceId, link_to_target: Link2Tgt) -> Self {
        Self::new(source, link_to_target.link, link_to_target.tgt)
    }

    pub fn get_link_to_target(&self) -> Link2Tgt {
        Link2Tgt::new(self.link.clone(), self.target.clone())
    }

    pub fn get_link_from_source(&self) -> LinkFrmSrc {
        LinkFrmSrc::new(self.link.clone(), self.source.clone())
    }
}

/// Allows to generate LinkFrmSrcToTarget from string tuple
/// (source, link, target)
impl From<(&str, &str, &str)> for LinkSrc2Tgt {
    fn from(value: (&str, &str, &str)) -> Self {
        LinkSrc2Tgt::new(value.0.into(), value.1.into(), Some(value.2.into()))
    }
}
