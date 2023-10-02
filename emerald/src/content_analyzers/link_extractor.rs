use std::iter::Filter;
use std::iter::Map;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::link::Link;

use super::md_extractor::{ContentType, MarkdownExtractorIterSource};

// --------------------------------------------------------------

pub trait LinkExtractorIterSource {
    type Iter: Iterator<Item = Link>;
    fn create_iter(&self, content: String) -> Self::Iter;
}

pub struct LinkExtractor<I: MarkdownExtractorIterSource> {
    content_iter_rc: Rc<I>,
}

impl<I: MarkdownExtractorIterSource> LinkExtractor<I> {
    pub fn new(content_iter_rc: Rc<I>) -> Self {
        Self { content_iter_rc }
    }
}

// ------------------------------------------------------------

impl<I: MarkdownExtractorIterSource> LinkExtractorIterSource for LinkExtractor<I> {
    type Iter = Map<Filter<I::Iter, fn(&ContentType) -> bool>, fn(ContentType) -> Link>;

    fn create_iter(&self, content: String) -> Self::Iter {
        fn filter_func(pred: &ContentType) -> bool {
            matches!(pred, ContentType::WikiLink(_))
        }

        fn map_func(x: ContentType) -> Link {
            match x {
                ContentType::WikiLink(link) => Link(link),
                _ => panic!(),
            }
        }
        self.content_iter_rc
            .create_iter(content)
            .filter(filter_func as _) //  as fn(&ContentType) -> bool also works
            .map(map_func as _)
    }
}
