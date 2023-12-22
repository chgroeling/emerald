use super::{
    resource_metadata::ResourceMetadata, resource_metadata_retriever::ResourceMetadataRetriever,
};
use crate::types;
use std::collections::HashMap;

pub struct ResourceMetadataMap {
    meta_data_map: HashMap<types::ResourceId, ResourceMetadata>,
}

impl ResourceMetadataMap {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, ResourceMetadata)>) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, ResourceMetadata>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid, meta_data).is_some() {
                panic!("This should not happen. No duplicate entries allowed.")
            }
        }
        Self { meta_data_map }
    }
}

impl ResourceMetadataRetriever for ResourceMetadataMap {
    fn retrieve(&self, md: &types::ResourceId) -> &ResourceMetadata {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map
            .get(md)
            .expect("Meta data was not stored. This should not happen.")
    }
}
