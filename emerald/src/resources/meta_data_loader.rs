use crate::types;
use types::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait MetaDataLoader {
    fn load(&self, resource_id: &types::ResourceId) -> Result<types::MetaData>;
}
