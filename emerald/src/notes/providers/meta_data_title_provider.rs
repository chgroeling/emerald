use std::rc::Rc;

use super::title_provider::TitleProvider;
use crate::error::Result;
use crate::model::note;
use crate::types;

pub struct MetaDataTitleProvider {
    meta_data_retriever: Rc<dyn note::MetaDataRetriever>,
}

impl MetaDataTitleProvider {
    pub fn new(meta_data_retriever: Rc<dyn note::MetaDataRetriever>) -> Self {
        Self {
            meta_data_retriever,
        }
    }
}
impl TitleProvider for MetaDataTitleProvider {
    fn get_title(&self, rid: &types::ResourceId) -> Result<String> {
        let meta_data = self.meta_data_retriever.retrieve(rid).clone();
        Ok(meta_data.file_stem)
    }
}
