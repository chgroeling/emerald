use std::rc::Rc;

use super::md_provider::MdProvider;
use crate::error::EmeraldError::*;
use crate::error::Result;
use crate::model::content;
use crate::model::note;
use crate::types;

pub struct ContentMdProvider {
    content_retriever: Rc<dyn content::MdContentRetriever>,
    meta_data_retriever: Rc<dyn note::MetaDataRetriever>,
}

impl ContentMdProvider {
    pub fn new(
        content_retriever: Rc<dyn content::MdContentRetriever>,
        meta_data_retriever: Rc<dyn note::MetaDataRetriever>,
    ) -> Self {
        Self {
            content_retriever,
            meta_data_retriever,
        }
    }
}
impl MdProvider for ContentMdProvider {
    fn get_markdown(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_retriever.retrieve(rid);

        // do not allow anything other than markdown files pass this point
        let types::FileType::Markdown(_) = meta_data.file_type else {
            panic!("This should not happen. A md resource id is not a markdown file.")
        };

        let res = self.content_retriever.retrieve(rid);
        res.0.clone()
    }
}
