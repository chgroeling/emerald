use super::title_provider::TitleProvider;
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct MetaDataTitleProvider {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
}

impl MetaDataTitleProvider {
    pub fn new(meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>) -> Self {
        Self {
            meta_data_retriever,
        }
    }
}
impl TitleProvider for MetaDataTitleProvider {
    fn get_title(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        meta_data.file_stem.clone()
    }
}
