use super::resource_object::ResourceObject;
use crate::error::Result;
use crate::types;
#[cfg(test)]
use mockall::{mock, predicate::*};

pub trait ResourceIdRetriever {
    fn retrieve(&self, ro: &ResourceObject) -> Result<types::ResourceId>;
}

#[cfg(test)]
mock! {

    pub ResourceIdResolver{}

    impl ResourceIdRetriever for ResourceIdResolver {
        fn retrieve(&self, ro: &ResourceObject) -> Result<types::ResourceId>;
    }

    impl Clone for ResourceIdResolver {
        fn clone(&self) -> Self;
    }

}
