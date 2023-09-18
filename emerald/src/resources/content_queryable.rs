use crate::types::Content;
use crate::types::ResourceId;

pub trait ContentQueryable {
    fn get(&self, resource_id: &ResourceId) -> Option<Content>;
}
