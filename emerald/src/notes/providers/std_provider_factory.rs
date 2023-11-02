use std::rc::Rc;

use super::{
    content_md_provider::ContentMdProvider, meta_data_string_provider::MetaDataStringProvider,
    meta_data_timestamp_provider::MetaDataTimestampProvider, provider_factory::ProviderFactory,
};
use crate::{model::content, model::note};

#[derive(Clone)]
pub struct StdProviderFactory {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl StdProviderFactory {
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
        content_retriever: Rc<dyn content::MdContentRetriever>,
    ) -> Self {
        Self {
            meta_data_retriever,
            content_retriever,
        }
    }
}

impl ProviderFactory for StdProviderFactory {
    fn create_title_provider(&self) -> Box<dyn super::string_provider::StringProvider> {
        Box::new(MetaDataStringProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.file_stem.to_owned(),
        ))
    }

    fn create_markdown_provider(&self) -> Box<dyn super::md_provider::MdProvider> {
        Box::new(ContentMdProvider::new(
            self.content_retriever.clone(),
            self.meta_data_retriever.clone(),
        ))
    }

    fn create_timestamp_created_provider(
        &self,
    ) -> Box<dyn super::timestamp_provider::TimestampProvider> {
        Box::new(MetaDataTimestampProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.created,
        ))
    }

    fn create_timestamp_modified_provider(
        &self,
    ) -> Box<dyn super::timestamp_provider::TimestampProvider> {
        Box::new(MetaDataTimestampProvider::new(
            self.meta_data_retriever.clone(),
            |meta_data| meta_data.modified,
        ))
    }
}
