use crate::model::note;
use crate::model::unique_id;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct UidMetadataRetrieverAdapter {
    metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
}

impl UidMetadataRetrieverAdapter {
    pub fn new(metadata_retriever: Rc<dyn note::NoteMetadataRetriever>) -> Self {
        Self { metadata_retriever }
    }
}

impl unique_id::UidMetadataRetriever<types::ResourceId> for UidMetadataRetrieverAdapter {
    fn retrieve(&self, rid: &types::ResourceId) -> Option<String> {
        let note_metadata = self.metadata_retriever.retrieve(rid);
        note_metadata.document.uid.to_owned()
    }
}
