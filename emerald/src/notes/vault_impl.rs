use std::rc::Rc;

use super::note::Note;
use super::NoteFactory;
use crate::model::note;
use crate::{types, Vault};

#[derive(Clone)]
pub struct VaultImpl<U, I>
where
    U: NoteFactory,
    I: Iterator<Item = types::ResourceId>,
{
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
    note_factory: U,
}

impl<U, I> VaultImpl<U, I>
where
    U: NoteFactory,
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>, note_factory: U) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            notes_iter_src,
            note_factory,
        }
    }

    pub fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let note_vec: Vec<Note> = self
            .notes_iter_src
            .create_iter()
            .map(|rid| self.note_factory.create_note(rid))
            .collect();

        note_vec.into_iter()
    }
}

impl<U, I> Vault for VaultImpl<U, I>
where
    U: NoteFactory,
    I: Iterator<Item = types::ResourceId>,
{
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        self.flat_iter()
    }
}
