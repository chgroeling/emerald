use crate::types::{Link, ResourceId};
use crate::Result;

pub enum Hint {
    #[allow(dead_code)]
    NoHint,
}
pub trait LinkQuerier {
    fn get(&self, link: &Link) -> Result<ResourceId> {
        self.get_with_hint(link, Hint::NoHint)
    }

    fn get_with_hint(&self, link: &Link, hint: Hint) -> Result<ResourceId>;
}
