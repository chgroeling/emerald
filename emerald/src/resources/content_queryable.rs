use crate::types::Content;
use crate::types::ResourceId;
use crate::Result;

pub trait ContentQueryable {
    fn query(&self, resource_id: &ResourceId) -> Result<Content>;
}
