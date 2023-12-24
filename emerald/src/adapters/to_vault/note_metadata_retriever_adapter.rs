use crate::model::note;
use crate::model::vault;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteMetadataRetrieverAdapter {
    metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
}

impl NoteMetadataRetrieverAdapter {
    pub fn new(meta_data_retriever: Rc<dyn note::NoteMetadataRetriever>) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
        }
    }
}

impl vault::NoteMetadataRetriever for NoteMetadataRetrieverAdapter {
    fn retrieve(
        &self,
        tgt: &vault::ResourceId,
    ) -> (String, vault::FilesystemMetadata, vault::DocumentMetadata) {
        let rid: types::ResourceId = tgt.clone().into();
        let note_metadata = self.metadata_retriever.retrieve(&rid);
        let filesystem_md: vault::FilesystemMetadata = note_metadata.into();
        let document_md: vault::DocumentMetadata = note_metadata.into();

        (note_metadata.title.clone(), filesystem_md, document_md)
    }
}
