use crate::types;

use super::{files_iter_src::FilesIterSrc, FileCount};

pub struct DefaultFileModel {
    file_index: Vec<types::ResourceId>,
}

impl DefaultFileModel {
    pub fn new(it_files: impl IntoIterator<Item = types::ResourceId>) -> DefaultFileModel {
        DefaultFileModel {
            file_index: it_files.into_iter().collect(),
        }
    }
}
impl FilesIterSrc for DefaultFileModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.file_index.clone().into_iter()
    }
}

impl FileCount for DefaultFileModel {
    fn count(&self) -> usize {
        self.file_index.len()
    }
}
