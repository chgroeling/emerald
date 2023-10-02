use std::rc::Rc;

use crate::maps::ResourceIdRetriever;

use super::{
    link_extractor::LinkExtractor, md_extractor::MarkdownExtractor,
    resource_id_extractor::ResourceIdExtractor,
    resource_id_extractor_iter_src::ResourceIdExtractorIterSrc, MdLinkAnalyzerIterSrc,
};

pub struct MdLinkAnalyzer {
    resource_id_extractor: Rc<ResourceIdExtractor<LinkExtractor<MarkdownExtractor>>>,
}

impl MdLinkAnalyzer {
    pub fn new(resource_id_retriever: Rc<dyn ResourceIdRetriever>) -> Self {
        let markdown_extractor = Rc::new(MarkdownExtractor::new());
        let link_extractor = Rc::new(LinkExtractor::new(markdown_extractor));
        let resource_id_extractor = Rc::new(ResourceIdExtractor::new(
            resource_id_retriever,
            link_extractor,
        ));
        Self {
            resource_id_extractor,
        }
    }
}

impl MdLinkAnalyzerIterSrc for MdLinkAnalyzer {
    type Iter =
        <ResourceIdExtractor<LinkExtractor<MarkdownExtractor>> as ResourceIdExtractorIterSrc>::Iter;

    fn create_iter(&self, content: String) -> Self::Iter {
        self.resource_id_extractor.create_iter(content)
    }
}
