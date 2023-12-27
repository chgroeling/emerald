use std::rc::Rc;

use super::{NotesIterSrc, ResourceId};

pub struct UidMapper<I>
where
    I: Iterator<Item = ResourceId>,
{
    notes_iter_src: Rc<dyn NotesIterSrc<Iter = I>>,
}

impl<I> UidMapper<I>
where
    I: Iterator<Item = ResourceId>,
{
    pub fn new(iter_src: Rc<dyn NotesIterSrc<Iter = I>>) -> Self {
        Self {
            notes_iter_src: iter_src,
        }
    }
}

impl<I> NotesIterSrc for UidMapper<I>
where
    I: Iterator<Item = ResourceId>,
{
    type Iter = std::vec::IntoIter<ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        let ret: Vec<_> = self.notes_iter_src.create_iter().collect();

        ret.into_iter()
    }
}
