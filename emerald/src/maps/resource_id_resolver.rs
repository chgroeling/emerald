use crate::types::{Link, ResourceId};
use crate::Result;

pub enum Hint {
    #[allow(dead_code)]
    NoHint,
}
pub trait ResourceIdResolver {
    // This is a resolver instead of a Retriever because the link is interpreted
    fn resolve(&self, link: &Link) -> Result<ResourceId> {
        self.resolve_with_hint(link, Hint::NoHint)
    }

    fn resolve_with_hint(&self, link: &Link, hint: Hint) -> Result<ResourceId>;
}
