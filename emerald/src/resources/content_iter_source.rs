use crate::types::{Content, ResourceId};

pub trait ContentIterSource {
    type Iter: Iterator<Item = (ResourceId, Content)>;
    fn iter(&self) -> Self::Iter;
}
