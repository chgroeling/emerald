use super::{
    resource_metadata::ResourceMetadata, resource_metadata_retriever::ResourceMetadataRetriever,
};
use crate::types;
use std::collections::HashMap;

pub struct ResourceMetadataMap {
    meta_data_map: HashMap<types::ResourceId, ResourceMetadata>,
}

impl ResourceMetadataMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = (&'a types::ResourceId, ResourceMetadata)> + 'a,
    ) -> Self {
        let mut meta_data_map = HashMap::<types::ResourceId, ResourceMetadata>::new();
        for (rid, meta_data) in it_src.into_iter() {
            if meta_data_map.insert(rid.to_owned(), meta_data).is_some() {
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
