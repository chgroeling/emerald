use crate::error::Result;
use crate::types;

pub trait ContentLoader {
    fn load(&self, rid: &types::ResourceId) -> Result<types::Content>;
}
