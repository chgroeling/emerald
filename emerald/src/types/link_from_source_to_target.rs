use super::{LinkToTarget, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds the source of a link and its target (the place where it points to).
/// Source and target are concrete ResourceIds which are pointing to files.
#[allow(dead_code)]
pub struct LinkFromSourceToTarget {
    pub source: ResourceId,
    pub link_to_target: LinkToTarget,
}

impl LinkFromSourceToTarget {
    pub fn new(source: ResourceId, link_to_target: LinkToTarget) -> Self {
        Self {
            source,
            link_to_target,
        }
    }
}
