use crate::types::ResourceId;

pub trait MdResourceIdsIterSource {
    type Iter: Iterator<Item = ResourceId>;
    fn md_iter(&self) -> Self::Iter;
}
