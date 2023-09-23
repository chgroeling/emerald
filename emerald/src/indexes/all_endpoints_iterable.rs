use crate::types::EndPoint;

pub trait AllEndpointsIterable {
    type Iter: Iterator<Item = EndPoint>;
    fn all_iter(&self) -> Self::Iter;
}
