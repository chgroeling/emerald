use crate::types::{EndPoint, ResourceId};
use crate::Result;

#[cfg(test)]
use mockall::{mock, predicate::*};

pub trait ResourceIdRetriever {
    fn resolve(&self, endpoint: &EndPoint) -> Result<ResourceId>;
}

#[cfg(test)]
mock! {

    pub ResourceIdResolver{}

    impl ResourceIdRetriever for ResourceIdResolver {
        fn resolve(&self, endpoint: &EndPoint) -> Result<ResourceId>;
    }

    impl Clone for ResourceIdResolver {
        fn clone(&self) -> Self;
    }

}
