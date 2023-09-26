use crate::types::LinkFromSourceToTarget;

#[cfg(test)]
use mockall::{automock, predicate::*};

/// Get an Iterator on on a list of `LinkFromSourceToTarget`objects.
#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<LinkFromSourceToTarget>;))]
pub trait SrcTgtIterable {
    type Iter: Iterator<Item = LinkFromSourceToTarget>;
    fn iter(&self) -> Self::Iter;
}
