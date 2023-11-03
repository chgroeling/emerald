use super::Provider;
use crate::model::content;
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct ContentMdProvider {
    content_retriever: Rc<dyn content::MdContentRetriever>,
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
}

impl ContentMdProvider {
    pub fn new(
        content_retriever: Rc<dyn content::MdContentRetriever>,
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    ) -> Self {
        Self {
            content_retriever,
            meta_data_retriever,
        }
    }
}
impl Provider<String> for ContentMdProvider {
    fn get(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_retriever.retrieve(rid);

        // do not allow anything other than markdown files pass this point
        let types::ResourceType::Markdown(_) = meta_data.resource_type else {
            panic!("This should not happen. A md resource id is not a markdown file.")
        };

        let res = self.content_retriever.retrieve(rid);
        res.0.clone()
    }
}
