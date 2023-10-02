use crate::types::Content;
use crate::types::ResourceId;
use crate::Result;

pub trait ContentQueryable {
    fn get(&self, resource_id: ResourceId) -> Result<Content>;
}
