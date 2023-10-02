use crate::types::{Link, ResourceId};
use crate::Result;

pub enum Hint {
    #[allow(dead_code)]
    NoHint,
}
pub trait LinkQuerier {
    fn query(&self, link: &Link) -> Result<ResourceId> {
        self.query_with_hint(link, Hint::NoHint)
    }

    fn query_with_hint(&self, link: &Link, hint: Hint) -> Result<ResourceId>;
}
