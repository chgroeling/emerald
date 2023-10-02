use crate::types::ResourceId;

pub trait ResourceIdsIterSrc {
    type Iter: Iterator<Item = ResourceId>;
    fn iter(&self) -> Self::Iter;
}
