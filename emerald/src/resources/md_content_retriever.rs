use crate::error::Result;
use crate::types;

pub trait MdContentRetriever {
    fn retrieve(&self, rid: &types::ResourceId) -> Result<&types::Content>;
}
