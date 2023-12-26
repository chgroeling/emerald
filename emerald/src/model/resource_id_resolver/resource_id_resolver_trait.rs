use crate::error::Result;
use crate::types;

pub enum Hint {
    #[allow(dead_code)]
    NoHint,
}
pub trait ResourceIdResolver {
    // This is a resolver instead of a Retriever because the link is interpreted
    fn resolve(&self, link: &types::Link) -> Result<&types::ResourceId> {
        self.resolve_with_hint(link, Hint::NoHint)
    }

    fn resolve_with_hint(&self, link: &types::Link, hint: Hint) -> Result<&types::ResourceId>;
}
