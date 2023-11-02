use super::{Provider, StringProvider, TimestampProvider};
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct MetaDataProvider<T, I>
where
    I: Fn(&types::MetaData) -> T,
{
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    from_metadata_to_t: I,
}

impl<T, I> MetaDataProvider<T, I>
where
    I: Fn(&types::MetaData) -> T,
{
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
        from_metadata_to_t: I,
    ) -> Self {
        Self {
            meta_data_retriever,
            from_metadata_to_t,
        }
    }
}
impl<I> TimestampProvider for MetaDataProvider<i64, I>
where
    I: Fn(&types::MetaData) -> i64,
{
    fn get(&self, rid: &types::ResourceId) -> i64 {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_t)(meta_data)
    }
}

impl<I> StringProvider for MetaDataProvider<String, I>
where
    I: Fn(&types::MetaData) -> String,
{
    fn get(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_t)(meta_data)
    }
}

impl<T, I> Provider<T> for MetaDataProvider<T, I>
where
    I: Fn(&types::MetaData) -> T,
{
    fn get(&self, rid: &types::ResourceId) -> T {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_t)(meta_data)
    }
}
