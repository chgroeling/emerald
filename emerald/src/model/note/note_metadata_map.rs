use super::{note_metadata::NoteMetadata, note_metadata_retriever::NoteMetadataRetriever};
use crate::types;
use std::collections::HashMap;

pub struct NoteMetadataMap {
    meta_data_map: HashMap<types::ResourceId, NoteMetadata>,
}

impl NoteMetadataMap {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, NoteMetadata)>) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, NoteMetadata>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid, meta_data).is_some() {
                panic!("This should not happen. No duplicate entries allowed.")
            }
        }
        Self { meta_data_map }
    }
}

impl NoteMetadataRetriever for NoteMetadataMap {
    fn retrieve(&self, md: &types::ResourceId) -> &NoteMetadata {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(md)
            .expect("Meta data was not stored. This should not happen.")
    }
}
