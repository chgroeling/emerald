use super::string_provider::StringProvider;
use crate::model::note;
use crate::types;
use std::rc::Rc;

pub struct MetaDataStringProvider<I>
where
    I: Fn(&types::MetaData) -> String,
{
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    from_metadata_to_string: I,
}

impl<I> MetaDataStringProvider<I>
where
    I: Fn(&types::MetaData) -> String,
{
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
        from_metadata_to_string: I,
    ) -> Self {
        Self {
            meta_data_retriever,
            from_metadata_to_string,
        }
    }
}
impl<I> StringProvider for MetaDataStringProvider<I>
where
    I: Fn(&types::MetaData) -> String,
{
    fn get_string(&self, rid: &types::ResourceId) -> String {
        let meta_data = self.meta_data_retriever.retrieve(rid);
        (self.from_metadata_to_string)(meta_data)
    }
}
