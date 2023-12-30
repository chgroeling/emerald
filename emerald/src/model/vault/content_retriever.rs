use super::ExResourceId;

pub trait ContentRetriever {
    fn retrieve(&self, rid: &ExResourceId) -> &str;
}
