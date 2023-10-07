use crate::types::ResourceId;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<ResourceId>;))]
pub trait ResourceIdsIterSrc {
    type Iter: Iterator<Item = ResourceId>;
    fn iter(&self) -> Self::Iter;
}
