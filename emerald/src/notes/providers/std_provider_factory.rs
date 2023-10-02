use std::rc::Rc;

use crate::resources::{content_loader::ContentLoader, meta_data_loader::MetaDataLoader};

use super::{
    content_markdown_provider::ContentMarkdownProvider,
    meta_data_title_provider::MetaDataTitleProvider, provider_factory::ProviderFactory,
};

pub struct StdProviderFactory<I, T>
where
    I: MetaDataLoader,
    T: ContentLoader,
{
    meta_data_loader: Rc<I>,
    content_queryable: Rc<T>,
}

impl<I, T> StdProviderFactory<I, T>
where
    I: MetaDataLoader,
    T: ContentLoader,
{
    pub fn new(meta_data_loader: Rc<I>, content_queryable: Rc<T>) -> Self {
        Self {
            meta_data_loader,
            content_queryable,
        }
    }
}

impl<I, T> ProviderFactory for StdProviderFactory<I, T>
where
    I: MetaDataLoader + 'static,
    T: ContentLoader + 'static,
{
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_loader.clone()))
    }

    fn create_markdown_provider(&self) -> Box<dyn super::markdown_provider::MarkdownProvider> {
        Box::new(ContentMarkdownProvider::new(self.content_queryable.clone()))
    }
}
