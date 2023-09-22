use crate::types::ResourceId;

pub trait MdResourceIdsIterable {
    type Iter: Iterator<Item = ResourceId>;
    fn md_iter(&self) -> Self::Iter;
}
