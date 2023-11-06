use crate::types;

pub struct ResourceRef {
    pub rid: types::ResourceId,
}

impl ResourceRef {
    pub fn new(rid: types::ResourceId) -> Self {
        Self { rid }
    }
}
