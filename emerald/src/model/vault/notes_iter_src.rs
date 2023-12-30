use super::ExResourceId;

pub trait NotesIterSrc {
    type Iter: Iterator<Item = ExResourceId>;
    fn create_iter(&self) -> Self::Iter;
}
