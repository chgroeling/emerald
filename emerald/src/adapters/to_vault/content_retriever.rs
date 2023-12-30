use crate::model::content;
use crate::model::vault;
use std::rc::Rc;

#[derive(Clone)]
pub struct ContentRetriever {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl ContentRetriever {
    pub fn new(meta_data_retriever: Rc<dyn content::MdContentRetriever>) -> Self {
        Self {
            content_retriever: meta_data_retriever,
        }
    }
}

impl vault::ContentRetriever for ContentRetriever {
    fn retrieve(&self, rid: &vault::VaultResourceId) -> &str {
        let content = self.content_retriever.retrieve(&rid.clone().into());

        &content.0
    }
}
