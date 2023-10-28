use crate::types;

use super::files_iter_src::FilesIterSrc;

pub struct DefaultFileModel {
    file_index: Vec<types::ResourceId>,
}

impl DefaultFileModel {
    pub fn new<'a>(it_files: impl IntoIterator<Item = &'a types::ResourceId>) -> DefaultFileModel {
        DefaultFileModel {
            file_index: it_files.into_iter().cloned().collect(),
        }
    }
}
impl FilesIterSrc for DefaultFileModel {
    type Iter = std::vec::IntoIter<types::ResourceId>;

    fn create_iter(&self) -> Self::Iter {
        self.file_index.clone().into_iter()
    }
}
