use crate::types;
use types::Result;

pub trait ContentLoader {
    fn load(&self, resource_id: &types::ResourceId) -> Result<types::Content>;
}
