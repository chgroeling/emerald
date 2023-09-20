use super::{Link, ResourceId};

/// This struct holds the origin of a link. The link itself and the
/// destination (dest) were it points to.
#[allow(dead_code)]
pub struct NoteLink {
    pub origin: ResourceId,
    pub link: Link,
    pub dest: Option<ResourceId>,
}
