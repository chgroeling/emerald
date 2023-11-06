use std::rc::Rc;

use super::get_backlinks::GetBacklinks;
use super::get_backlinks_impl::GetBacklinksImpl;
use super::get_links::{GetLinks, GetLinksResult};
use super::get_links_impl::GetLinksImpl;
use super::note::Note;
use super::NoteFactory;
use crate::model::{link, note, resource};
use crate::{types, Vault};

#[derive(Clone)]
pub struct VaultImpl<I, U: GetLinks = GetLinksImpl, L: GetBacklinks = GetBacklinksImpl>
where
    I: Iterator<Item = types::ResourceId>,
{
    note_factory: Rc<dyn NoteFactory>,
    notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
    get_links: U,
    get_backlinks: L,
}

impl<I> VaultImpl<I, GetLinksImpl, GetBacklinksImpl>
where
    I: Iterator<Item = types::ResourceId>,
{
    pub fn new(
        note_factory: Rc<dyn NoteFactory>,
        notes_iter_src: Rc<dyn note::NotesIterSrc<Iter = I>>,
        tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
        src_link_retriever: Rc<dyn link::SrcIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    ) -> Self
    where
        I: Iterator<Item = types::ResourceId>,
    {
        let get_links = GetLinksImpl::new(tgt_link_retriever.clone(), res_meta_data_ret.clone());
        let get_backlinks = GetBacklinksImpl::new(src_link_retriever.clone());
        Self {
            notes_iter_src,
            note_factory,
            get_links,
            get_backlinks,
        }
    }
}

impl<I, U: GetLinks> Vault for VaultImpl<I, U>
where
    I: Iterator<Item = types::ResourceId>,
{
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let note_vec: Vec<Note> = self
            .notes_iter_src
            .create_iter()
            .map(|rid| self.note_factory.create_note(rid))
            .collect();

        note_vec.into_iter()
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let factory_clone = self.note_factory.clone();
        Box::new(
            self.get_links
                .get_links_of(note)
                .filter_map(move |f| match f {
                    GetLinksResult::LinkToNote(rid) => Some(factory_clone.create_note(rid)),
                    GetLinksResult::LinkToResource(_) => None,
                }),
        )
    }

    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let factory_clone = self.note_factory.clone();
        Box::new(
            self.get_backlinks
                .get_backlinks_of(note)
                .map(move |f| factory_clone.create_note(f)),
        )
    }
}
