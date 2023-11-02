use super::TimestampProvider;
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct MetaDataTimestampProvider<I>
where
    I: Fn(&types::MetaData) -> i64,
{
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    from_metadata_to_timestamp: I,
}

impl<I> MetaDataTimestampProvider<I>
where
    I: Fn(&types::MetaData) -> i64,
{
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
        from_metadata_to_timestamp: I,
    ) -> Self {
        Self {
            meta_data_retriever,
            from_metadata_to_timestamp,
        }
    }
}
impl<I> TimestampProvider for MetaDataTimestampProvider<I>
where
    I: Fn(&types::MetaData) -> i64,
{
    fn get(&self, rid: &types::ResourceId) -> i64 {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_timestamp)(meta_data)
    }
}
