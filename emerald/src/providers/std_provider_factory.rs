use std::rc::Rc;

use crate::resources::{content_queryable::ContentQueryable, meta_data_loader::MetaDataLoader};

use super::{
    meta_data_title_provider::MetaDataTitleProvider, provider_factory::ProviderFactory,
    std_content_provider::StdContentProvider,
};

pub struct StdProviderFactory {
    meta_data_loader: Rc<dyn MetaDataLoader>,
    content_queryable: Rc<dyn ContentQueryable>,
}

impl StdProviderFactory {
    pub fn new(
        meta_data_loader: Rc<dyn MetaDataLoader>,
        content_queryable: Rc<dyn ContentQueryable>,
    ) -> Self {
        Self {
            meta_data_loader,
            content_queryable,
        }
    }
}

impl ProviderFactory for StdProviderFactory {
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_loader.clone()))
    }

    fn create_content_provider(&self) -> Box<dyn super::content_provider::ContentProvider> {
        Box::new(StdContentProvider::new(self.content_queryable.clone()))
    }
}
