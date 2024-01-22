use crate::model::note;
use crate::model::unique_id;
use crate::model::vault;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteMetadataRetriever {
    metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
    uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>,
}

impl NoteMetadataRetriever {
    /// Creates a new `NoteMetadataRetriever`.
    ///
    /// # Arguments
    ///
    /// * `metadata_retriever`: An `Rc` pointer to an object implementing `note::NoteMetadataRetriever`.
    /// * `uid_retriever`: An `Rc` pointer to an object implementing `unique_id::UidRetriever`.
    ///
    /// # Returns
    ///
    /// A new instance of `NoteMetadataRetriever`.
    pub fn new(
        metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
        uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>,
    ) -> Self {
        Self {
            metadata_retriever,
            uid_retriever,
        }
    }
}

impl vault::NoteMetadataRetriever<unique_id::Uid> for NoteMetadataRetriever {
    fn retrieve(
        &self,
        tgt: &unique_id::Uid,
    ) -> (String, vault::FilesystemMetadata, vault::DocumentMetadata) {
        let rid = self
            .uid_retriever
            .get_rid_from_uid(tgt)
            .expect("Resource Id not found");

        let note_metadata = self.metadata_retriever.retrieve(&rid);
        let filesystem_md: vault::FilesystemMetadata = note_metadata.into();
        let document_md: vault::DocumentMetadata = note_metadata.into();

        (note_metadata.title.clone(), filesystem_md, document_md)
    }
}
