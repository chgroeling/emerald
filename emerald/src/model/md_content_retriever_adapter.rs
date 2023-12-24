use super::content;
use super::vault;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct MdContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl MdContentRetrieverAdapter {
    pub fn new(meta_data_retriever: Rc<dyn content::MdContentRetriever>) -> Self {
        Self {
            content_retriever: meta_data_retriever,
        }
    }
}

impl vault::MdContentRetriever for MdContentRetrieverAdapter {
    fn retrieve(&self, rid: &types::ResourceId) -> &str {
        let content = self.content_retriever.retrieve(rid);

        &content.0
    }
}
