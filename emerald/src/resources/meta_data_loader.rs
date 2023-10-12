use crate::types::MetaData;
use crate::types::ResourceId;
use crate::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait MetaDataLoader {
    fn load(&self, resource_id: &ResourceId) -> Result<MetaData>;
}
