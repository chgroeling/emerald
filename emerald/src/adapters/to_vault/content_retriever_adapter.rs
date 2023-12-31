use crate::model::content;
use crate::model::vault;
use std::rc::Rc;

/// Adapts a generic markdown content retriever to the `vault::ContentRetriever` interface.
///
/// This struct allows the use of any markdown content retriever that implements
/// `content::MdContentRetriever` to be used wherever a `vault::ContentRetriever` is needed.
#[derive(Clone)]
pub struct ContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl ContentRetrieverAdapter {
    /// Creates a new `ContentRetrieverAdapter`.
    ///
    /// # Arguments
    ///
    /// * `content_retriever`: An `Rc` pointer to an object implementing `content::MdContentRetriever`.
    ///
    /// # Returns
    ///
    /// A new instance of `ContentRetrieverAdapter`.
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
