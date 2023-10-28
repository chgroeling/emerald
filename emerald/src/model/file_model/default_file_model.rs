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
    pub fn get_file_iterator<'a>(&'a self) -> impl Iterator<Item = &'a types::ResourceId> {
        self.file_index.iter()
    }
}
