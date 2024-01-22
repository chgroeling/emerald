use crate::model::content;
use crate::model::unique_id;
use crate::model::vault;
use crate::types;
use std::rc::Rc;

/// Adapts a generic markdown content retriever to the `vault::MdContentRetriever` interface.
///
/// This struct allows the use of any markdown content retriever that implements
/// `content::MdContentRetriever` to be used wherever a `vault::MdContentRetriever`
/// is needed.
#[derive(Clone)]
pub struct MdContentRetrieverAdapter {
    content_retriever: Rc<dyn content::MdContentRetriever>,
    uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>,
}

impl MdContentRetrieverAdapter {
    /// Creates a new `MdContentRetrieverAdapter`.
    ///
    /// # Arguments
    ///
    /// * `content_retriever`: An `Rc` pointer to an object implementing `content::MdContentRetriever`.
    /// * `uid_retriever`: An `Rc` pointer to an object implementing `unique_id::UidRetriever`.
    ///
    /// # Returns
    ///
    /// A new instance of `MdContentRetrieverAdapter`.
    pub fn new(
        content_retriever: Rc<dyn content::MdContentRetriever>,
        uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>,
    ) -> Self {
        Self {
            content_retriever,
            uid_retriever,
        }
    }
}

impl vault::MdContentRetriever<unique_id::Uid> for MdContentRetrieverAdapter {
    fn retrieve(&self, uid: &unique_id::Uid) -> &str {
        let rid = self
            .uid_retriever
            .get_rid_from_uid(uid)
            .expect("Resource Id not found");
        let content = self.content_retriever.retrieve(&rid.0.clone().into());

        &content.0
    }
}
