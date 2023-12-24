use std::rc::Rc;

use super::get_backlinks::GetBacklinks;
use super::get_links::GetLinks;
use super::link_query_result::LinkQueryResult;
use super::note::Note;
use super::note_types::NoteTypes;
use super::vault_trait::Vault;
use super::NoteFactory;
use crate::model::note;
use crate::types;

#[derive(Clone)]
pub struct VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    note_factory: Rc<dyn NoteFactory>,
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
    get_backlinks: Rc<dyn GetBacklinks>,
    get_links: Rc<dyn GetLinks>,
}

impl<I> VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(
        note_factory: Rc<dyn NoteFactory>,
        notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
        get_backlinks: Rc<dyn GetBacklinks>,
        get_links: Rc<dyn GetLinks>,
    ) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        Self {
            notes_iter_src,
            note_factory,
            get_links,
            get_backlinks,
        }
    }
}

impl<I> Vault for VaultImpl<I>
where
    I: Iterator<Item = types::ResourceId>,
{
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let note_vec: Vec<Note> = self
            .notes_iter_src
            .create_iter()
            .map(|rid| self.note_factory.create_note(rid.into()))
            .collect();

        note_vec.into_iter()
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>> {
        let factory_clone = self.note_factory.clone();
        Box::new(self.get_links.get_links_of(note).map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => NoteTypes::Note(factory_clone.create_note(rid)),
            LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
        }))
    }

    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>> {
        let factory_clone = self.note_factory.clone();
        Box::new(
            self.get_backlinks
                .get_backlinks_of(note)
                .map(move |f| match f {
                    LinkQueryResult::LinkToNote(rid) => {
                        NoteTypes::Note(factory_clone.create_note(rid))
                    }
                    LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
                }),
        )
    }
}
