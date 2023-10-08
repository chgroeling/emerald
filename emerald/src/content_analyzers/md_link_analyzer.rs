use std::rc::Rc;

use crate::{maps::ResourceIdRetriever, types::Content};

use super::{
    link_extractor::LinkExtractor, md_extractor::MarkdownExtractor,
    resource_id_extractor::ResourceIdExtractor,
    resource_id_extractor_iter_src::ResourceIdExtractorIterSrc, MdLinkAnalyzerIterSrc,
};

pub struct MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever,
{
    resource_id_extractor: Rc<ResourceIdExtractor<LinkExtractor<MarkdownExtractor>, U>>,
}

impl<U> MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever,
{
    pub fn new(resource_id_retriever: Rc<U>) -> Self {
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

impl<U> MdLinkAnalyzerIterSrc for MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever + 'static,
{
    type Iter =
        <ResourceIdExtractor<LinkExtractor<MarkdownExtractor>,U> as ResourceIdExtractorIterSrc>::Iter;

    fn iter(&self, content: Content) -> Self::Iter {
        self.resource_id_extractor.iter(content)
    }
}
