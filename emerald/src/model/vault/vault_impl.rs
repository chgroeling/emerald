use super::get_backlinks::GetBacklinks;
use super::get_links::GetLinks;
use super::link_query_result::LinkQueryResult;
use super::note::Note;
use super::note_types::NoteTypes;
use super::resource_id_trait::ResourceIdTrait;
use super::uid_retriever::UidRetriever;
use super::uid_trait::UidTrait;
use super::vault_trait::Vault;
use super::NoteFactory;
use super::{MdContentRetriever, NoteFactoryImpl, NoteMetadataRetriever};
use std::rc::Rc;

#[derive(Clone)]
pub struct VaultImpl<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    note_factory: Rc<NoteFactoryImpl<U>>,
    get_backlinks: Rc<dyn GetBacklinks<T>>,
    get_links: Rc<dyn GetLinks<T>>,
    uid_retriever: Rc<dyn UidRetriever<T, U>>,
}

impl<T, U> VaultImpl<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    pub fn new(
        metadata_retriever: Rc<dyn NoteMetadataRetriever<U>>,
        content_retriever: Rc<dyn MdContentRetriever<U>>,
        get_backlinks: Rc<dyn GetBacklinks<T>>,
        get_links: Rc<dyn GetLinks<T>>,
        uid_retriever: Rc<dyn UidRetriever<T, U>>,
    ) -> Self {
        let note_factory = Rc::new(NoteFactoryImpl::<U>::new(
            metadata_retriever,
            content_retriever,
        ));
        Self {
            note_factory,
            get_links,
            get_backlinks,
            uid_retriever,
        }
    }
}

impl<T, U> Vault<T, U> for VaultImpl<T, U>
where
    T: ResourceIdTrait + 'static,
    U: UidTrait + 'static,
{
    fn get_note(&self, uid: &U) -> Note<U> {
        self.note_factory.create_note(uid)
    }

    fn get_links_of(&self, note: &Note<U>) -> Box<dyn Iterator<Item = NoteTypes<T, U>> + 'static> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_retriever.clone();
        let rid = self
            .uid_retriever
            .get_rid_from_uid(&note.uid)
            .expect("Should exist");

        let link_iter = self.get_links.get_links_of(rid);

        Box::new(link_iter.map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => {
                let link_uid = uid_map_clone.get_uid_from_rid(&rid).expect("Should exist");
                NoteTypes::Note(factory_clone.create_note(link_uid))
            }
            LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
        }))
    }

    fn get_backlinks_of(
        &self,
        note: &Note<U>,
    ) -> Box<dyn Iterator<Item = NoteTypes<T, U>> + 'static> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_retriever.clone();
        let rid = self
            .uid_retriever
            .get_rid_from_uid(&note.uid)
            .expect("Should exist");
        let backlinks_iter = self.get_backlinks.get_backlinks_of(rid);

        Box::new(backlinks_iter.map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => {
                let link_uid = uid_map_clone.get_uid_from_rid(&rid).expect("Should exist");
                NoteTypes::Note(factory_clone.create_note(link_uid))
            }
            LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid),
        }))
    }
}
