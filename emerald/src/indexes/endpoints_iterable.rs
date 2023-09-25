use crate::types::EndPoint;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock(type Iter=std::vec::IntoIter<EndPoint>;))]
pub trait EndpointsIterable {
    type Iter: Iterator<Item = EndPoint>;
    fn iter(&self) -> Self::Iter;
}
