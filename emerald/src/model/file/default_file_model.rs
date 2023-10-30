use super::file_count::FileCount;
use super::file_meta_data_map::FileMetaDataMap;
use super::file_meta_data_retriever::FileMetaDataRetriever;
use super::files_iter_src::FilesIterSrc;

use crate::types;

pub struct DefaultFileModel {
    file_index: Vec<types::ResourceId>,
    meta_data_map: FileMetaDataMap,
}

impl DefaultFileModel {
    pub fn new(
        it_files: impl IntoIterator<Item = (types::ResourceId, types::MetaData)>,
    ) -> DefaultFileModel {
        let files: Vec<_> = it_files.into_iter().collect();
        DefaultFileModel {
            file_index: files.iter().map(|f| f.0.clone()).collect(),
            meta_data_map: FileMetaDataMap::new(files.into_iter()),
        }
    }
}
impl FilesIterSrc for DefaultFileModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.file_index.clone().into_iter()
    }
}

impl FileMetaDataRetriever for DefaultFileModel {
    fn retrieve(&self, tgt: &types::ResourceId) -> &types::MetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map.retrieve(tgt)
    }
}

impl FileCount for DefaultFileModel {
    fn count(&self) -> usize {
        self.file_index.len()
    }
}
