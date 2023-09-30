use std::rc::Rc;

use crate::resources::meta_data_loader::MetaDataLoader;

use super::ResourceId;

pub struct Note {
    pub resource_id: ResourceId,
    meta_data_loader: Rc<dyn MetaDataLoader>,
}

impl Note {
    pub fn new(resource_id: ResourceId, meta_data_loader: Rc<dyn MetaDataLoader>) -> Self {
        Self {
            resource_id,
            meta_data_loader,
        }
    }
}
