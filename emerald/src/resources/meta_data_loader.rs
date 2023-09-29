use crate::types::meta_data::MetaData;
use crate::types::ResourceId;
use crate::Result;

pub trait MetaDataLoader {
    fn load(&self, resource_id: ResourceId) -> Result<MetaData>;
}
