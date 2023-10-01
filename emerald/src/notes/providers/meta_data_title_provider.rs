use std::rc::Rc;

use super::title_provider::TitleProvider;

use crate::resources::meta_data_loader::MetaDataLoader;
use crate::types::ResourceId;

pub struct MetaDataTitleProvider {
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl MetaDataTitleProvider {
    pub fn new(meta_data_loader: Rc<dyn MetaDataLoader>) -> Self {
        Self { meta_data_loader }
    }
}
impl TitleProvider for MetaDataTitleProvider {
    fn get_title(&self, resource_id: &ResourceId) -> String {
        let meta_data = self.meta_data_loader.load(resource_id).unwrap();
        meta_data.file_stem
    }
}
