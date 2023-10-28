use crate::types;

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

impl<'a> IntoIterator for &'a DefaultFileModel {
    type Item = &'a types::ResourceId;

    type IntoIter = std::slice::Iter<'a, types::ResourceId>;

    fn into_iter(self) -> Self::IntoIter {
        let file_idx = &self.file_index;
        file_idx.into_iter()
    }
}
