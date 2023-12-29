use super::resource_count::ResourceCount;
use super::resource_iter_src::ResourceIterSrc;
use super::resource_metadata::ResourceMetadata;
use super::resource_metadata_map::ResourceMetadataMap;
use super::resource_metadata_retriever::ResourceMetadataRetriever;
use crate::types;

pub struct DefaultResourceModel {
    resource_index: Vec<types::ResourceId>,
    meta_data_map: ResourceMetadataMap,
}

impl DefaultResourceModel {
    pub fn new<'a>(
        it_files: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)> + 'a,
    ) -> DefaultResourceModel {
        let files: Vec<_> = it_files
            .into_iter()
            .map(|(rid, fs_md)| (rid, ResourceMetadata::from(fs_md)))
            .collect();

        DefaultResourceModel {
            resource_index: files.iter().map(|f| f.0.clone()).collect(),
            meta_data_map: ResourceMetadataMap::new(files),
        }
    }
}
impl ResourceIterSrc for DefaultResourceModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.resource_index.clone().into_iter()
    }
}

impl ResourceMetadataRetriever for DefaultResourceModel {
    fn retrieve(&self, tgt: &types::ResourceId) -> &ResourceMetadata {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map.retrieve(tgt)
    }
}

impl ResourceCount for DefaultResourceModel {
    fn count(&self) -> usize {
        self.resource_index.len()
    }
}
