use crate::types;

pub trait MdContentRetriever {
    fn retrieve(&self, rid: &types::ResourceId) -> &str;
}
