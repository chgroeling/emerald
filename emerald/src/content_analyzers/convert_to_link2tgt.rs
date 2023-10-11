use crate::{
    maps::ResourceIdRetriever,
    types::{Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn convert_to_link2tgt<'a>(
    link_iter: impl Iterator<Item = Link> + 'static,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> + 'a {
    ResourceIdExtractorIterator {
        input_iter: link_iter,
        resource_id_retriever,
    }
}

pub struct ResourceIdExtractorIterator<'a, I, U>
where
    I: Iterator<Item = Link>,
    U: ResourceIdRetriever,
{
    input_iter: I,
    resource_id_retriever: &'a U,
}

impl<'a, I, U> Iterator for ResourceIdExtractorIterator<'a, I, U>
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
