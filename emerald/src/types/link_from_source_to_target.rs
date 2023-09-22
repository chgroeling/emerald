use super::{Link, LinkFromSource, LinkToTarget, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds the source of a link and its target (the place where it points to).
/// Source and target are concrete ResourceIds which are pointing to files.
#[allow(dead_code)]
pub struct LinkFromSourceToTarget {
    pub source: ResourceId,
    pub link: Link,
    pub target: Option<ResourceId>,
}

impl LinkFromSourceToTarget {
    pub fn new(source: ResourceId, link: Link, target: Option<ResourceId>) -> Self {
        Self {
            source,
            link,
            target,
        }
    }

    pub fn from_link_to_target(source: ResourceId, link_to_target: LinkToTarget) -> Self {
        Self::new(source, link_to_target.link, link_to_target.target)
    }

    pub fn get_link_to_target(&self) -> LinkToTarget {
        LinkToTarget::new(self.link.clone(), self.target.clone())
    }

    pub fn get_link_from_source(&self) -> LinkFromSource {
        LinkFromSource::new(self.link.clone(), self.source.clone())
    }
}
