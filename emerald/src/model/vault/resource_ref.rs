use super::resource_id::ResourceId;

pub struct ResourceRef {
    pub rid: ResourceId,
}

impl ResourceRef {
    pub fn new(rid: ResourceId) -> Self {
        Self { rid }
    }
}
