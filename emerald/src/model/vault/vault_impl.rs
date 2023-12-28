use super::adapter_to_uid::adapter_to_uid;
use super::get_backlinks::GetBacklinks;
use super::get_links::GetLinks;
use super::link_query_result::LinkQueryResult;
use super::note::Note;
use super::note_types::NoteTypes;
use super::uid_map::UidMap;
use super::vault_trait::Vault;
use super::{ContentRetriever, NoteFactoryImpl, NoteMetadataRetriever};
use super::{NoteFactory, ResourceId};
use std::rc::Rc;

#[derive(Clone)]
pub struct VaultImpl {
    note_rid_list: Vec<ResourceId>,
    note_factory: Rc<dyn NoteFactory>,
    get_backlinks: Rc<dyn GetBacklinks>,
    get_links: Rc<dyn GetLinks>,
}

impl VaultImpl {
    pub fn new<'a>(
        note_rid_iter: impl IntoIterator<Item = ResourceId>,
        metadata_retriever: Rc<dyn NoteMetadataRetriever>,
        content_retriever: Rc<dyn ContentRetriever>,
        get_backlinks: Rc<dyn GetBacklinks>,
        get_links: Rc<dyn GetLinks>,
    ) -> Self {
        let note_factory = Rc::new(NoteFactoryImpl::new(metadata_retriever, content_retriever));
        let note_list: Vec<_> = note_rid_iter.into_iter().collect();
        Self {
            note_rid_list: note_list,
            note_factory,
            get_links,
            get_backlinks,
        }
    }
}

impl Vault for VaultImpl {
    fn flat_iter(&self) -> std::vec::IntoIter<Note> {
        let mut uid_map = UidMap::new();
        let note_vec: Vec<Note> = adapter_to_uid(self.note_rid_list.iter(), &mut uid_map)
            .map(|rid| self.note_factory.create_note(rid))
            .collect();

        note_vec.into_iter()
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>> {
        let factory_clone = self.note_factory.clone();
        Box::new(self.get_links.get_links_of(note).map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => NoteTypes::Note(factory_clone.create_note(&rid)),
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
                        NoteTypes::Note(factory_clone.create_note(&rid))
                    }
                    LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
                }),
        )
    }
}
