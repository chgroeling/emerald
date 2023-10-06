use std::iter::Filter;
use std::iter::Map;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::link::Link;
use crate::types::Content;

use super::content_type::ContentType;
use super::link_extractor_iter_src::LinkExtractorIterSrc;
use super::md_extractor_iter_src::MarkdownExtractorIterSrc;

pub struct LinkExtractor<I: MarkdownExtractorIterSrc> {
    content_iter_rc: Rc<I>,
}

impl<I: MarkdownExtractorIterSrc> LinkExtractor<I> {
    pub fn new(content_iter_rc: Rc<I>) -> Self {
        Self { content_iter_rc }
    }
}

impl<I: MarkdownExtractorIterSrc> LinkExtractorIterSrc for LinkExtractor<I> {
    type Iter = Map<Filter<I::Iter, fn(&ContentType) -> bool>, fn(ContentType) -> Link>;

    fn iter(&self, content: Content) -> Self::Iter {
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
            .iter(content)
            .filter(filter_func as _) //  as fn(&ContentType) -> bool also works
            .map(map_func as _)
    }
}
