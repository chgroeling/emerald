use std::rc::Rc;

use crate::maps::LinkRetriever;

use super::{
    link_extractor::LinkExtractor, md_extractor::MarkdownExtractor,
    resource_id_extractor::ResourceIdExtractor,
    resource_id_extractor_iter_src::ResourceIdExtractorIterSrc, MdLinkAnalyzerIterSrc,
};

type ResourceIdExtractorIteratorImpl =
    <ResourceIdExtractor<LinkExtractor<MarkdownExtractor>> as ResourceIdExtractorIterSrc>::Iter;

pub struct MdLinkAnalyzer {
    resource_id_extractor: Rc<ResourceIdExtractor<LinkExtractor<MarkdownExtractor>>>,
}

impl MdLinkAnalyzer {
    pub fn new(link_retriever: Rc<dyn LinkRetriever>) -> Self {
        let markdown_extractor = Rc::new(MarkdownExtractor::new());
        let link_extractor = Rc::new(LinkExtractor::new(markdown_extractor));
        let resource_id_extractor =
            Rc::new(ResourceIdExtractor::new(link_retriever, link_extractor));
        Self {
            resource_id_extractor,
        }
    }
}

impl MdLinkAnalyzerIterSrc for MdLinkAnalyzer {
    type Iter = ResourceIdExtractorIteratorImpl;

    fn create_iter(&self, content: String) -> Self::Iter {
        self.resource_id_extractor.create_iter(content)
    }
}
