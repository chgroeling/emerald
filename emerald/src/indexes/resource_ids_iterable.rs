use crate::types::ResourceId;

pub trait ResourceIdsIterable {
    type Iter: Iterator<Item = ResourceId>;
    fn iter(&self) -> Self::Iter;
}
