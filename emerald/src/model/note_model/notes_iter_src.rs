use crate::types;

pub trait NotesIterSrc {
    type Iter: Iterator<Item = types::ResourceId>;
    fn iter(&self) -> Self::Iter;
}
