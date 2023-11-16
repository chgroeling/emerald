use super::{note_meta_data::NoteMetaData, note_meta_data_retriever::NoteMetaDataRetriever};
use crate::types;
use std::collections::HashMap;

pub struct NoteMetaDataMap {
    meta_data_map: HashMap<types::ResourceId, NoteMetaData>,
}

impl NoteMetaDataMap {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, NoteMetaData)>) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, NoteMetaData>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid, meta_data).is_some() {
                panic!("This should not happen. No duplicate entries allowed")
            }
        }
        Self { meta_data_map }
    }
}

impl NoteMetaDataRetriever for NoteMetaDataMap {
    fn retrieve(&self, md: &types::ResourceId) -> &NoteMetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(md)
            .expect("Meta data was not stored. This should not happen")
    }
}
