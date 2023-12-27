use std::{cell::Cell, rc::Rc};

use super::{NotesIterSrc, ResourceId};

pub struct NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    notes_iter_src: Rc<dyn NotesIterSrc<Iter = I>>,
    next_uid: Cell<u32>,
}

impl<I> NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    pub fn new(iter_src: Rc<dyn NotesIterSrc<Iter = I>>) -> Self {
        Self {
            notes_iter_src: iter_src,
            next_uid: Cell::new(0),
        }
    }
}

impl<I> NotesIterSrc for NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    type Iter = std::vec::IntoIter<ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        let ret: Vec<_> = self.notes_iter_src.create_iter().collect();

        let uid = self.next_uid.get() + 1;

        self.next_uid.set(uid);
        ret.into_iter()
    }
}
