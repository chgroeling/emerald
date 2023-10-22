use crate::error::Result;
use crate::types;

pub trait MdContentRetriever {
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<&types::Content>;
}
