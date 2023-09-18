use crate::types::ResourceId;

pub trait AllResourceIdsIterSource {
    type Iter: Iterator<Item = ResourceId>;
    fn all_iter(&self) -> Self::Iter;
}
