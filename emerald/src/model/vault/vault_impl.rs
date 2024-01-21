use super::get_backlinks::GetBacklinks;
use super::get_links::GetLinks;
use super::link_query_result::LinkQueryResult;
use super::note::Note;
use super::note_types::NoteTypes;
use super::resource_id_trait::ResourceIdTrait;
use super::vault_trait::Vault;
use super::NoteFactory;
use super::{MdContentRetriever, NoteFactoryImpl, NoteMetadataRetriever};
use crate::model::unique_id::UidRetriever;
use std::rc::Rc;

#[derive(Clone)]
pub struct VaultImpl<T>
where
    T: ResourceIdTrait,
{
    note_factory: Rc<NoteFactoryImpl<T>>,
    get_backlinks: Rc<dyn GetBacklinks<T>>,
    get_links: Rc<dyn GetLinks<T>>,
    uid_retriever: Rc<dyn UidRetriever<T>>,
}

impl<T> VaultImpl<T>
where
    T: ResourceIdTrait,
{
    pub fn new(
        metadata_retriever: Rc<dyn NoteMetadataRetriever<T>>,
        content_retriever: Rc<dyn MdContentRetriever<T>>,
        get_backlinks: Rc<dyn GetBacklinks<T>>,
        get_links: Rc<dyn GetLinks<T>>,
        uid_retriever: Rc<dyn UidRetriever<T>>,
    ) -> Self {
        let note_factory = Rc::new(NoteFactoryImpl::<T>::new(
            metadata_retriever,
            content_retriever,
            uid_retriever.clone(),
        ));
        Self {
            note_factory,
            get_links,
            get_backlinks,
            uid_retriever,
        }
    }
}

impl<T> Vault<T> for VaultImpl<T>
where
    T: ResourceIdTrait,
{
    fn get_note(&self, rid: &T) -> Note {
        let uid = self
            .uid_retriever
            .get_uid_from_rid(rid)
            .expect("Unknown ExResourceId");
        self.note_factory.create_note(uid)
    }

    fn get_resource_id(&self, note: &Note) -> Option<&T> {
        self.uid_retriever.get_rid_from_uid(&note.uid)
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>> + 'static> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_retriever.clone();
        let rid = self
            .uid_retriever
            .get_rid_from_uid(&note.uid)
            .expect("Should exist");
        Box::new(self.get_links.get_links_of(rid).map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => {
                let link_uid = uid_map_clone.get_uid_from_rid(&rid).expect("Should exist");
                NoteTypes::Note(factory_clone.create_note(link_uid))
            }
            LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
        }))
    }

    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>> + 'static> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_retriever.clone();
        let rid = self
            .uid_retriever
            .get_rid_from_uid(&note.uid)
            .expect("Should exist");
        Box::new(
            self.get_backlinks
                .get_backlinks_of(rid)
                .map(move |f| match f {
                    LinkQueryResult::LinkToNote(rid) => {
                        let link_uid = uid_map_clone.get_uid_from_rid(&rid).expect("Should exist");
                        NoteTypes::Note(factory_clone.create_note(link_uid))
                    }
                    LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
                }),
        )
    }
}
