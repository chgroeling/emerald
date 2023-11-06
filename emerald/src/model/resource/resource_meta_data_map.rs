use super::{
    resource_meta_data::ResourceMetaData, resource_meta_data_retriever::ResourceMetaDataRetriever,
};
use crate::types;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ResourceMetaDataMap {
    meta_data_map: HashMap<types::ResourceId, ResourceMetaData>,
}

impl ResourceMetaDataMap {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, ResourceMetaData)>) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, ResourceMetaData>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid, meta_data).is_some() {
                panic!("This should not happen. No duplicate entries allowed")
            }
        }
        Self { meta_data_map }
    }
}

impl ResourceMetaDataRetriever for ResourceMetaDataMap {
    fn retrieve(&self, md: &types::ResourceId) -> &ResourceMetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(md)
            .expect("Meta data was not stored. This should not happen")
    }
}
