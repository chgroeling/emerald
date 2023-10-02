use crate::types::{Content, ResourceId};

pub trait ContentIterSrc {
    type Iter: Iterator<Item = (ResourceId, Content)>;
    fn iter(&self) -> Self::Iter;
}
