use crate::types;

pub trait FilesIterSrc {
    type Iter: Iterator<Item = types::ResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
