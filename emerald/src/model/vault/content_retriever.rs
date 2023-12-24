use super::ResourceId;

pub trait MdContentRetriever {
    fn retrieve(&self, rid: &ResourceId) -> &str;
}
