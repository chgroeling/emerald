use std::rc::Rc;

use crate::resources::meta_data_loader::MetaDataLoader;

use super::{meta_data_title_provider::MetaDataTitleProvider, provider_factory::ProviderFactory};

pub struct NoteProviders {
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl NoteProviders {
    pub fn new(meta_data_loader: Rc<dyn MetaDataLoader>) -> Self {
        Self { meta_data_loader }
    }
}

impl ProviderFactory for NoteProviders {
    fn create_title_provider(&self) -> Box<dyn super::title_provider::TitleProvider> {
        Box::new(MetaDataTitleProvider::new(self.meta_data_loader.clone()))
    }
}
