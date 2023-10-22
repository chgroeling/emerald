use crate::types;
use types::Result;

pub trait MdContentRetriever {
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<&types::Content>;
}
