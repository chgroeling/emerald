use crate::model::content;
use crate::model::note_updater;
use crate::types;
use std::rc::Rc;

/// Adapts a generic markdown content retriever to the `note_updater::MdContentRetriever` interface.
///
/// This struct allows the use of any markdown content retriever that implements
/// `content::MdContentRetriever` to be used wherever a `note_updater::MdContentRetriever`
/// is needed.
#[derive(Clone)]
pub struct MdContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl MdContentRetrieverAdapter {
    /// Creates a new `MdContentRetrieverAdapter`.
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

impl note_updater::MdContentRetriever<types::ResourceId> for MdContentRetrieverAdapter {
    fn retrieve(&self, rid: &types::ResourceId) -> &str {
        let content = self.content_retriever.retrieve(rid);

        &content.0
    }
}
