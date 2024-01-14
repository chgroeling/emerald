use super::ex_resource_id::VaultResourceId;
use super::get_backlinks::GetBacklinks;
use super::get_links::GetLinks;
use super::link_query_result::LinkQueryResult;
use super::note::Note;
use super::note_types::NoteTypes;
use super::uid_map::UidMap;
use super::vault_trait::Vault;
use super::NoteFactory;
use super::{MdContentRetriever, NoteFactoryImpl, NoteMetadataRetriever};
use std::rc::Rc;

#[derive(Clone)]
pub struct VaultImpl<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    note_factory: Rc<dyn NoteFactory>,
    get_backlinks: Rc<dyn GetBacklinks<T>>,
    get_links: Rc<dyn GetLinks<T>>,
    uid_map: Rc<UidMap<T>>,
}

impl<T> VaultImpl<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone + 'static,
{
    pub fn new(
        note_rid_iter: impl IntoIterator<Item = VaultResourceId<T>>,
        metadata_retriever: Rc<dyn NoteMetadataRetriever<T>>,
        content_retriever: Rc<dyn MdContentRetriever<T>>,
        get_backlinks: Rc<dyn GetBacklinks<T>>,
        get_links: Rc<dyn GetLinks<T>>,
    ) -> Self {
        let mut uid_map = UidMap::<T>::new();

        for rid in note_rid_iter.into_iter() {
            uid_map.assign_uid(&rid);
        }

        let rc_uid_map = Rc::new(uid_map);
        let note_factory = Rc::new(NoteFactoryImpl::<T>::new(
            metadata_retriever,
            content_retriever,
            rc_uid_map.clone(),
        ));
        Self {
            note_factory,
            get_links,
            get_backlinks,
            uid_map: rc_uid_map,
        }
    }
}

impl<T> Vault<T> for VaultImpl<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone + 'static,
{
    fn get_note(&self, rid: &T) -> Note {
        let vrid = VaultResourceId::<T>(rid.clone());
        let uid = self
            .uid_map
            .get_uid_from_rid(&vrid)
            .expect("Unknown ExResourceId");
        self.note_factory.create_note(uid)
    }

    fn get_resource_id(&self, note: &Note) -> Option<&T> {
        self.uid_map.get_rid_from_uid(&note.uid).map(|f| &f.0)
    }

    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>>> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_map.clone();
        let rid = self
            .uid_map
            .get_rid_from_uid(&note.uid)
            .expect("Should exist");
        Box::new(self.get_links.get_links_of(rid).map(move |f| match f {
            LinkQueryResult::LinkToNote(rid) => {
                let link_uid = uid_map_clone.get_uid_from_rid(&rid).expect("Should exist");
                NoteTypes::Note(factory_clone.create_note(link_uid))
            }
            LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid.0),
        }))
    }

    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>>> {
        let factory_clone = self.note_factory.clone();
        let uid_map_clone = self.uid_map.clone();
        let rid = self
            .uid_map
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
                    LinkQueryResult::LinkToResource(rid) => NoteTypes::ResourceRef(rid.0),
                }),
        )
    }
}
