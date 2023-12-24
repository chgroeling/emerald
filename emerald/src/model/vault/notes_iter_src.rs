use super::ResourceId;

pub trait NotesIterSrc {
    type Iter: Iterator<Item = ResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
