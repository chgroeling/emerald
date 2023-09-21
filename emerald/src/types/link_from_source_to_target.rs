use super::{LinkToTarget, ResourceId};

#[derive(Debug, Clone)]
/// This struct holds the source of links and its target (the place where it points to) and the
/// Resource Id of the source, the place where it stems from
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
