use std::{cell::RefCell, rc::Rc};

use super::uid_map::UidMap;
use super::{NotesIterSrc, ResourceId};

pub struct NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    notes_iter_src: Rc<dyn NotesIterSrc<Iter = I>>,
    uid_map: Rc<RefCell<UidMap>>,
}

impl<I> NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    pub fn new(iter_src: Rc<dyn NotesIterSrc<Iter = I>>) -> Self {
        let uid_map = Rc::new(RefCell::new(UidMap::new()));
        Self {
            notes_iter_src: iter_src,
            uid_map,
        }
    }
}

impl<I> NotesIterSrc for NoteWithUidIterSrc<I>
where
    I: Iterator<Item = ResourceId>,
{
    type Iter = std::vec::IntoIter<ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        let uid_map_cpy = self.uid_map.clone();
        let ret: Vec<_> = self
            .notes_iter_src
            .create_iter()
            .map(move |rid| {
                uid_map_cpy.borrow_mut().get_or_assign_uid(&rid);
                rid
            })
            .collect();

        ret.into_iter()
    }
}
