use std::rc::Rc;

use super::{
    content_md_provider::ContentMdProvider, meta_data_created_provider::MetaDataCreatedProvider,
    meta_data_modified_provider::MetaDataModifiedProvider,
    meta_data_title_provider::MetaDataTitleProvider, provider_factory::ProviderFactory,
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
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_retriever.clone()))
    }

    fn create_markdown_provider(&self) -> Box<dyn super::md_provider::MdProvider> {
        Box::new(ContentMdProvider::new(
            self.content_retriever.clone(),
            self.meta_data_retriever.clone(),
        ))
    }

    fn create_created_provider(&self) -> Box<dyn super::timestamp_provider::TimestampProvider> {
        Box::new(MetaDataCreatedProvider::new(
            self.meta_data_retriever.clone(),
        ))
    }

    fn create_modified_provider(&self) -> Box<dyn super::timestamp_provider::TimestampProvider> {
        Box::new(MetaDataModifiedProvider::new(
            self.meta_data_retriever.clone(),
        ))
    }
}
