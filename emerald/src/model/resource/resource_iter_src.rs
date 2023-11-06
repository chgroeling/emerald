use crate::types;

pub trait ResourceIterSrc {
    type Iter: Iterator<Item = types::ResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
