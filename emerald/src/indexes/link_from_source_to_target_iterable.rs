use crate::types::LinkFromSourceToTarget;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<LinkFromSourceToTarget>;))]
pub trait LinkFromSourceToTargetIterable {
    type Iter: Iterator<Item = LinkFromSourceToTarget>;
    fn iter(&self) -> Self::Iter;
}
