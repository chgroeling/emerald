use crate::error::Result;
use crate::types;

pub trait ContentLoader {
    fn load(&self, resource_id: &types::ResourceId) -> Result<types::Content>;
}
