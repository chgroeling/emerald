use crate::types::{EndPoint, ResourceId};
use crate::Result;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait ResourceIdRetriever {
    fn query(&self, resource_id: &ResourceId) -> Result<EndPoint>;
}
