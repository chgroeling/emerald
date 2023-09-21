use std::rc::Rc;

use crate::maps::LinkQueryable;

use super::{
    link_extractor::LinkExtractor,
    md_extractor::MarkdownExtractor,
    resource_id_extractor::{ResourceIdExtractor, ResourceIdExtractorIterSource},
    MdLinkAnalyzerIterSource,
};

type IMarkdownIteratorSource = MarkdownExtractor;
type ILinkExtractorIteratorSource = LinkExtractor<IMarkdownIteratorSource>;

type ResourceIdExtractorIteratorImpl =
    <ResourceIdExtractor<ILinkExtractorIteratorSource> as ResourceIdExtractorIterSource>::Iter;

pub struct MdLinkAnalyzer {
    resource_id_extractor: Rc<ResourceIdExtractor<ILinkExtractorIteratorSource>>,
}

impl MdLinkAnalyzer {
    pub fn new(link_queryable: Rc<dyn LinkQueryable>) -> Self {
        let markdown_extractor = Rc::new(MarkdownExtractor::new());
        let link_extractor = Rc::new(LinkExtractor::new(markdown_extractor));
        let resource_id_extractor =
            Rc::new(ResourceIdExtractor::new(link_queryable, link_extractor));
        Self {
            resource_id_extractor,
        }
    }
}

impl MdLinkAnalyzerIterSource for MdLinkAnalyzer {
    type Iter = ResourceIdExtractorIteratorImpl;

    fn create_iter(&self, content: String) -> Self::Iter {
        self.resource_id_extractor.create_iter(content)
    }
}
