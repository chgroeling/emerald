use crate::types;

pub trait NotesIterSrc {
    type Iter: Iterator<Item = types::ResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
