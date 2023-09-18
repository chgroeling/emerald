use crate::types::{Content, ResourceId};

pub trait ContentIterSource<'a> {
    type Iter: Iterator<Item = &'a (ResourceId, Content)>;
    fn iter(&'a self) -> Self::Iter;
}
