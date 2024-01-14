use crate::model::note;
use crate::model::vault;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteMetadataRetriever {
    metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
}

impl NoteMetadataRetriever {
    pub fn new(meta_data_retriever: Rc<dyn note::NoteMetadataRetriever>) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
        }
    }
}

impl vault::NoteMetadataRetriever<vault::ExResourceId> for NoteMetadataRetriever {
    fn retrieve(
        &self,
        tgt: &vault::VaultResourceId<vault::ExResourceId>,
    ) -> (String, vault::FilesystemMetadata, vault::DocumentMetadata) {
        let rid: types::ResourceId = tgt.clone().0.into();
        let note_metadata = self.metadata_retriever.retrieve(&rid);
        let filesystem_md: vault::FilesystemMetadata = note_metadata.into();
        let document_md: vault::DocumentMetadata = note_metadata.into();

        (note_metadata.title.clone(), filesystem_md, document_md)
    }
}
