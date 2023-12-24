use super::ResourceId;

pub trait ContentRetriever {
    fn retrieve(&self, rid: &ResourceId) -> &str;
}
