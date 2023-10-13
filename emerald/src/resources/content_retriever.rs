use crate::types::Content;
use crate::types::ResourceId;
use crate::Result;

pub trait ContentRetriever {
    fn load(&self, resource_id: &ResourceId) -> Result<Content>;
}
