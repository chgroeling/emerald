use super::note::{DocumentMetadata, FilesystemMetadata};
use super::resource_id::ResourceId;
use crate::model::note;
use crate::types;
use std::rc::Rc;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait NoteMetadataRetriever {
    fn retrieve(&self, tgt: &ResourceId) -> (String, FilesystemMetadata, DocumentMetadata);
}

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

impl NoteMetadataRetriever for NoteMetadataRetrieverAdapter {
    fn retrieve(&self, tgt: &ResourceId) -> (String, FilesystemMetadata, DocumentMetadata) {
        let rid: types::ResourceId = tgt.clone().into();
        let note_metadata = self.metadata_retriever.retrieve(&rid);
        let filesystem_md: FilesystemMetadata = note_metadata.into();
        let document_md: DocumentMetadata = note_metadata.into();

        (note_metadata.title.clone(), filesystem_md, document_md)
    }
}
