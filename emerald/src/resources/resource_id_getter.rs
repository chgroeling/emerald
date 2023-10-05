use crate::types::{EndPoint, ResourceId};
use crate::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ResourceIdGetter {
    fn get(&self, endpoint: &EndPoint) -> Result<ResourceId>;
}
