use std::rc::Rc;

use crate::resources::meta_data_loader::MetaDataLoader;

use super::ResourceId;

pub struct Note {
    resource_id: ResourceId,
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl Note {
    pub fn new(resource_id: ResourceId, meta_data_loader: Rc<dyn MetaDataLoader>) -> Self {
        Self {
            resource_id,
            meta_data_loader,
        }
    }

    pub fn title(&self) -> String {
        let meta_data = self.meta_data_loader.load(&self.resource_id).unwrap();
        meta_data.file_stem
    }
}
