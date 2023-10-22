use crate::types;
#[cfg(test)]
use mockall::{mock, predicate::*};
use types::Result;

pub trait ResourceIdRetriever {
    fn retrieve(&self, endpoint: &types::EndPoint) -> Result<types::ResourceId>;
}

#[cfg(test)]
mock! {

    pub ResourceIdResolver{}

    impl ResourceIdRetriever for ResourceIdResolver {
        fn retrieve(&self, endpoint: &types::EndPoint) -> Result<types::ResourceId>;
    }

    impl Clone for ResourceIdResolver {
        fn clone(&self) -> Self;
    }

}
