use super::timestamp_provider::TimestampProvider;
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct MetaDataModifiedProvider {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
}

impl MetaDataModifiedProvider {
    pub fn new(meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>) -> Self {
        Self {
            meta_data_retriever,
        }
    }
}
impl TimestampProvider for MetaDataModifiedProvider {
    fn get_timestamp(&self, rid: &types::ResourceId) -> i64 {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        meta_data.modified
    }
}
