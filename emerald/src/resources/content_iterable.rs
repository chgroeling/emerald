use crate::types::{Content, ResourceId};

pub trait ContentIterable {
    type Iter: Iterator<Item = (ResourceId, Content)>;
    fn iter(&self) -> Self::Iter;
}
