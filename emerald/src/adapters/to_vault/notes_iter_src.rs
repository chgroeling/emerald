use crate::model::note;
use crate::model::vault;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NotesIterSrc<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
}

impl<I> NotesIterSrc<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>) -> Self {
        Self { notes_iter_src }
    }
}

impl<I> vault::NotesIterSrc for NotesIterSrc<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    type Iter = std::vec::IntoIter<vault::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        let vec: Vec<vault::ResourceId> = self
            .notes_iter_src
            .create_iter()
            .map(|f| f.into())
            .collect();

        vec.into_iter()
    }
}
