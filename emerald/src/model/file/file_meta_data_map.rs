use super::file_meta_data_retriever::FileMetaDataRetriever;
use crate::types;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FileMetaDataMap {
    meta_data_map: HashMap<types::ResourceId, types::MetaData>,
}

impl FileMetaDataMap {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, types::MetaData)>) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, types::MetaData>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid, meta_data).is_some() {
                panic!("This should not happen. No duplicate entries allowed")
            }
        }
        Self { meta_data_map }
    }
}

impl FileMetaDataRetriever for FileMetaDataMap {
    fn retrieve(&self, md: &types::ResourceId) -> &types::MetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(md)
            .expect("Meta data was not stored. This should not happen")
    }
}