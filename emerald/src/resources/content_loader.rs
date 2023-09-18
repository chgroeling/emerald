use crate::types::Content;
use crate::types::ResourceId;
use crate::Result;

pub trait ContentLoader {
    fn load(&self, resource_id: &ResourceId) -> Result<Content>;
}
