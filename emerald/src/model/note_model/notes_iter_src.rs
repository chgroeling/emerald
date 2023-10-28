use crate::types;

pub trait NotesIterSrc<'a> {
    type Iter: Iterator<Item = &'a types::ResourceId>;
    fn iter(&'a self) -> Self::Iter;
}
