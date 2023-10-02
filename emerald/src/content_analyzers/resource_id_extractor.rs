use std::rc::Rc;

use crate::{
    maps::LinkQuerier,
    types::{Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::link_extractor::LinkExtractorIterSource;

pub trait ResourceIdExtractorIterSource {
    type Iter: Iterator<Item = Link2Tgt>;
    fn create_iter(&self, content: String) -> Self::Iter;
}

// --------------------------------------------------------------------------

pub struct ResourceIdExtractorIterator<Iter> {
    input_iter: Iter,
    link_queryable: Rc<dyn LinkQuerier>,
}

impl<Iter: Iterator<Item = Link>> Iterator for ResourceIdExtractorIterator<Iter> {
    type Item = Link2Tgt;

    fn next(&mut self) -> Option<Self::Item> {
        let link_candidate = self.input_iter.next()?;
        if let Ok(resource_id) = self.link_queryable.get(&link_candidate) {
            Some(Link2Tgt::new(link_candidate, Some(resource_id)))
        } else {
            Some(Link2Tgt::new(link_candidate, None))
        }
    }
}

// --------------------------------------------------------------------------

pub struct ResourceIdExtractor<I: LinkExtractorIterSource> {
    link_queryable: Rc<dyn LinkQuerier>,
    link_extractor: Rc<I>,
}

impl<I: LinkExtractorIterSource> ResourceIdExtractor<I> {
    pub fn new(link_queryable: Rc<dyn LinkQuerier>, link_extractor: Rc<I>) -> Self {
        Self {
            link_queryable,
            link_extractor,
        }
    }
}

impl<I: LinkExtractorIterSource> ResourceIdExtractorIterSource for ResourceIdExtractor<I> {
    type Iter = ResourceIdExtractorIterator<I::Iter>;

    fn create_iter(&self, content: String) -> Self::Iter {
        ResourceIdExtractorIterator {
            input_iter: self.link_extractor.create_iter(content),
            link_queryable: self.link_queryable.clone(),
        }
    }
}
