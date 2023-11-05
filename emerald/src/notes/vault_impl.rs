use std::rc::Rc;

use super::note::Note;
use super::NoteFactory;
use crate::model::note;
use crate::{types, Vault};

#[derive(Clone)]
pub struct VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
    tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
    src_link_retriever: Rc<dyn note::SrcIterRetriever>,
    note_factory: Rc<dyn NoteFactory>,
}

impl<I> VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(
        notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
        note_factory: Rc<dyn NoteFactory>,
        tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
        src_link_retriever: Rc<dyn note::SrcIterRetriever>,
    ) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            notes_iter_src,
            note_factory,
            tgt_link_retriever,
            src_link_retriever,
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

impl<I> Vault for VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        self.flat_iter()
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let factory_clone = self.note_factory.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                Some(factory_clone.create_note(valid_tgt))
            } else {
                None
            }
        }))
    }

    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.src_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let factory_clone = self.note_factory.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            let valid_src = i.src;
            Some(factory_clone.create_note(valid_src))
        }))
    }
}
