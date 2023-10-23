use crate::error::Result;
use crate::types;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait MetaDataLoader {
    fn load(&self, rid: &types::ResourceId) -> Result<types::MetaData>;
}
