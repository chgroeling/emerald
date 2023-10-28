use std::rc::Rc;

use super::{
    content_md_provider::ContentMdProvider, meta_data_title_provider::MetaDataTitleProvider,
    provider_factory::ProviderFactory,
};
use crate::{model::note_model, resources};

#[derive(Clone)]
pub struct StdProviderFactory<T>
where
    T: resources::MdContentRetriever + Clone,
{
    meta_data_retriever: Rc<dyn note_model::MetaDataRetriever>,
    content_loader: T,
}

impl<T> StdProviderFactory<T>
where
    T: resources::MdContentRetriever + Clone,
{
    pub fn new(
        meta_data_retriever: Rc<dyn note_model::MetaDataRetriever>,
        content_loader: T,
    ) -> Self {
        Self {
            meta_data_retriever,
            content_loader,
        }
    }
}

impl<T> ProviderFactory for StdProviderFactory<T>
where
    T: resources::MdContentRetriever + 'static + Clone,
{
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_retriever.clone()))
    }

    fn create_markdown_provider(&self) -> Box<dyn super::md_provider::MdProvider> {
        Box::new(ContentMdProvider::new(
            self.content_loader.clone(),
            self.meta_data_retriever.clone(),
        ))
    }
}
