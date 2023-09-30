use super::ResourceId;

pub struct Note {
    pub resource_id: ResourceId,
}

impl Note {
    pub fn new(resource_id: ResourceId) -> Self {
        Self { resource_id }
    }
}
