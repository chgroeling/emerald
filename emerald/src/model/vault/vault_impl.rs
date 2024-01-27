use super::note::Note;
use super::uid_trait::UidTrait;
use super::vault_trait::Vault;
use super::NoteFactory;
use super::{MdContentRetriever, NoteFactoryImpl, NoteMetadataRetriever};
use std::rc::Rc;

#[derive(Clone)]
pub struct VaultImpl<U>
where
    U: UidTrait,
{
    note_factory: Rc<NoteFactoryImpl<U>>,
}

impl<U> VaultImpl<U>
where
    U: UidTrait,
{
    pub fn new(
        metadata_retriever: Rc<dyn NoteMetadataRetriever<U>>,
        content_retriever: Rc<dyn MdContentRetriever<U>>,
    ) -> Self {
        let note_factory = Rc::new(NoteFactoryImpl::<U>::new(
            metadata_retriever,
            content_retriever,
        ));
        Self { note_factory }
    }
}

impl<U> Vault<U> for VaultImpl<U>
where
    U: UidTrait + 'static,
{
    fn get_note(&self, uid: &U) -> Note<U> {
        self.note_factory.create_note(uid)
    }
}
