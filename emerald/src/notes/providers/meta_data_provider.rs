use super::Provider;
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

impl<T, I> Provider<T> for MetaDataProvider<T, I>
where
    I: Fn(&types::MetaData) -> T,
{
    fn get(&self, rid: &types::ResourceId) -> T {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_t)(meta_data)
    }
}
