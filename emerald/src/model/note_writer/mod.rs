mod ex_resource_id;
mod md_content_retriever;

use std::rc::Rc;

pub use self::ex_resource_id::ExResourceId;
pub use self::md_content_retriever::MdContentRetriever;
pub struct NoteWriter {
    content_retriever: Rc<dyn MdContentRetriever>,
}

impl NoteWriter {
    pub fn new(content_retriever: Rc<dyn MdContentRetriever>) -> Self {
        Self { content_retriever }
    }
}
