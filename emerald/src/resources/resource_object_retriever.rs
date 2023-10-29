use super::resource_object::ResourceObject;
use crate::error::Result;
use crate::types;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ResourceObjectRetriever {
    fn retrieve(&self, rid: &types::ResourceId) -> Result<ResourceObject>;
}
