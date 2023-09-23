use crate::types::ResourceId;

pub trait ResourceIdsIterable {
    type Iter: Iterator<Item = ResourceId>;
    fn md_iter(&self) -> Self::Iter;
}
