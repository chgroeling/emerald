use crate::types::Content;
use crate::types::ResourceId;
use crate::Result;

pub trait MdContentRetriever {
    fn retrieve(&self, resource_id: &ResourceId) -> Result<&Content>;
}
