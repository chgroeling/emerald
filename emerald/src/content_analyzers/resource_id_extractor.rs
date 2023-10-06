use std::rc::Rc;

use crate::{
    maps::ResourceIdRetriever,
    types::{Content, Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::{
    link_extractor_iter_src::LinkExtractorIterSrc,
    resource_id_extractor_iter_src::ResourceIdExtractorIterSrc,
};

pub struct ResourceIdExtractorIterator<Iter> {
    input_iter: Iter,
    resource_id_retriever: Rc<dyn ResourceIdRetriever>,
}

impl<Iter: Iterator<Item = Link>> Iterator for ResourceIdExtractorIterator<Iter> {
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

// --------------------------------------------------------------------------

pub struct ResourceIdExtractor<I: LinkExtractorIterSrc> {
    resource_id_retriever: Rc<dyn ResourceIdRetriever>,
    link_extractor: Rc<I>,
}

impl<I: LinkExtractorIterSrc> ResourceIdExtractor<I> {
    pub fn new(resource_id_retriever: Rc<dyn ResourceIdRetriever>, link_extractor: Rc<I>) -> Self {
        Self {
            resource_id_retriever,
            link_extractor,
        }
    }
}

impl<I: LinkExtractorIterSrc> ResourceIdExtractorIterSrc for ResourceIdExtractor<I> {
    type Iter = ResourceIdExtractorIterator<I::Iter>;

    fn iter(&self, content: Content) -> Self::Iter {
        ResourceIdExtractorIterator {
            input_iter: self.link_extractor.iter(content),
            resource_id_retriever: self.resource_id_retriever.clone(),
        }
    }
}
