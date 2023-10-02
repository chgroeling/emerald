use crate::types::{Link, ResourceId};
use crate::Result;

pub enum Hint {
    #[allow(dead_code)]
    NoHint,
}
pub trait LinkRetriever {
    fn retrieve(&self, link: &Link) -> Result<ResourceId> {
        self.retrieve_with_hint(link, Hint::NoHint)
    }

    fn retrieve_with_hint(&self, link: &Link, hint: Hint) -> Result<ResourceId>;
}
