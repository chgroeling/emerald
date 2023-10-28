use crate::types;

pub trait FilesIterSrc {
    type Iter: Iterator<Item = types::ResourceId>;
    fn iter(&self) -> Self::Iter;
}
