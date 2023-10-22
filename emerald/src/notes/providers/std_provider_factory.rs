use crate::resources::{MdContentRetriever, MetaDataLoader};

use super::{
    content_md_provider::ContentMdProvider, meta_data_title_provider::MetaDataTitleProvider,
    provider_factory::ProviderFactory,
};

#[derive(Clone)]
pub struct StdProviderFactory<I, T>
where
    I: MetaDataLoader + Clone,
    T: MdContentRetriever + Clone,
{
    meta_data_loader: I,
    content_loader: T,
}

impl<I, T> StdProviderFactory<I, T>
where
    I: MetaDataLoader + Clone,
    T: MdContentRetriever + Clone,
{
    pub fn new(meta_data_loader: I, content_loader: T) -> Self {
        Self {
            meta_data_loader,
            content_loader,
        }
    }
}

impl<I, T> ProviderFactory for StdProviderFactory<I, T>
where
    I: MetaDataLoader + 'static + Clone,
    T: MdContentRetriever + 'static + Clone,
{
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_loader.clone()))
    }

    fn create_markdown_provider(&self) -> Box<dyn super::md_provider::MdProvider> {
        Box::new(ContentMdProvider::new(
            self.content_loader.clone(),
            self.meta_data_loader.clone(),
        ))
    }
}
