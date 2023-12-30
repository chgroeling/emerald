use crate::model::content;
use crate::model::vault;
use std::rc::Rc;

#[derive(Clone)]
pub struct ContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl ContentRetrieverAdapter {
    pub fn new(content_retriever: Rc<dyn content::MdContentRetriever>) -> Self {
        Self { content_retriever }
    }
}

impl vault::ContentRetriever for ContentRetrieverAdapter {
    fn retrieve(&self, rid: &vault::ExResourceId) -> &str {
        let content = self.content_retriever.retrieve(&rid.clone().into());

        &content.0
    }
}
