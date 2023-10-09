use crate::{
    maps::ResourceIdRetriever,
    types::{Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct ResourceIdExtractorIterator<I, U>
where
    I: Iterator<Item = Link>,
    U: ResourceIdRetriever,
{
    input_iter: I,
    resource_id_retriever: U,
}

impl<I, U> Iterator for ResourceIdExtractorIterator<I, U>
where
    I: Iterator<Item = Link>,
    U: ResourceIdRetriever,
{
    type Item = Link2Tgt;

    fn next(&mut self) -> Option<Self::Item> {
        let link_candidate = self.input_iter.next()?;
        if let Ok(resource_id) = self.resource_id_retriever.retrieve(&link_candidate) {
            Some(Link2Tgt::new(link_candidate, Some(resource_id)))
        } else {
            Some(Link2Tgt::new(link_candidate, None))
        }
    }
}

// TODO: IMPL als Argument für TEmplate new
pub fn convert_to_resource_id(
    link_iter: impl Iterator<Item = Link>,
    resource_id_retriever: impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> {
    ResourceIdExtractorIterator {
        input_iter: link_iter,
        resource_id_retriever,
    }
}
