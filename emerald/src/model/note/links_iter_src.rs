use crate::types;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<types::LinkSrc2Tgt>;))]
pub trait LinksIterSrc {
    type Iter: Iterator<Item = types::LinkSrc2Tgt>;
    fn create_iter(&self) -> Self::Iter;
}
