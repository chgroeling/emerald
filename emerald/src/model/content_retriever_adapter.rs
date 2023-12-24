use super::content;
use super::vault;
use std::rc::Rc;

#[derive(Clone)]
pub struct ContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl ContentRetrieverAdapter {
    pub fn new(meta_data_retriever: Rc<dyn content::MdContentRetriever>) -> Self {
        Self {
            content_retriever: meta_data_retriever,
        }
    }
}

impl vault::ContentRetriever for ContentRetrieverAdapter {
    fn retrieve(&self, rid: &vault::ResourceId) -> &str {
        let content = self.content_retriever.retrieve(&rid.clone().into());

        &content.0
    }
}
