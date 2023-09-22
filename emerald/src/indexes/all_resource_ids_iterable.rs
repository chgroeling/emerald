use crate::types::ResourceId;

pub trait AllResourceIdsIterable {
    type Iter: Iterator<Item = ResourceId>;
    fn all_iter(&self) -> Self::Iter;
}
