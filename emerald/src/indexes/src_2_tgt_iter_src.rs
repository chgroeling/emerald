use crate::types::LinkSrc2Tgt;

#[cfg(test)]
use mockall::{automock, predicate::*};

/// Get an Iterator on a list of `LinkSrc2Tgt`objects.
#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<LinkSrc2Tgt>;))]
pub trait Src2TgtIterSrc {
    type Iter: Iterator<Item = LinkSrc2Tgt>;
    fn iter(&self) -> Self::Iter;
}
