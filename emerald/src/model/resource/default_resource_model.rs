use super::resource_count::ResourceCount;
use super::resource_iter_src::ResourceIterSrc;
use super::resource_meta_data::ResourceMetaData;
use super::resource_meta_data_map::ResourceMetaDataMap;
use super::resource_meta_data_retriever::ResourceMetaDataRetriever;
use crate::types;

pub struct DefaultResourceModel {
    file_index: Vec<types::ResourceId>,
    meta_data_map: ResourceMetaDataMap,
}

impl DefaultResourceModel {
    pub fn new(
        it_files: impl IntoIterator<Item = (types::ResourceId, types::FilesystemMetaData)>,
    ) -> DefaultResourceModel {
        let files: Vec<(_, ResourceMetaData)> =
            it_files.into_iter().map(|f| (f.0, f.1.into())).collect();

        DefaultResourceModel {
            file_index: files.iter().map(|f| f.0.clone()).collect(),
            meta_data_map: ResourceMetaDataMap::new(files),
        }
    }
}
impl ResourceIterSrc for DefaultResourceModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.file_index.clone().into_iter()
    }
}

impl ResourceMetaDataRetriever for DefaultResourceModel {
    fn retrieve(&self, tgt: &types::ResourceId) -> &ResourceMetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map.retrieve(tgt)
    }
}

impl ResourceCount for DefaultResourceModel {
    fn count(&self) -> usize {
        self.file_index.len()
    }
}
