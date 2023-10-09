use crate::{
    maps::{resource_id_link_map::ResourceIdLinkMap, ResourceIdRetriever},
    types::{Content, Link2Tgt},
};

use super::{
    link_extractor::extract_links, md_extractor::extract_content_types,
    resource_id_extractor::extract_resource, MdLinkAnalyzerIterSrc,
};

pub fn extract_md_links(
    content: Content,
    resource_id_retriever: impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> {
    let content_type_iter = extract_content_types(content);
    let link_iter = extract_links(content_type_iter);
    extract_resource(link_iter, resource_id_retriever)
}

type AbstractLinkExtractor<TResRetriever, C> = fn(Content, TResRetriever) -> C;

pub fn arg_extract<TResRetriever, C>(
    md_link_extractor: AbstractLinkExtractor<TResRetriever, C>,
    content: Content,
    resource_id_retriever: TResRetriever,
) -> impl Iterator<Item = Link2Tgt>
where
    TResRetriever: ResourceIdRetriever,
    C: Iterator<Item = Link2Tgt>,
{
    md_link_extractor(content, resource_id_retriever)
}

#[derive(Clone)]
pub struct MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever + Clone,
{
    resource_id_retriever: U,
}

impl<U> MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever + Clone,
{
    pub fn new(resource_id_retriever: U) -> Self {
        Self {
            resource_id_retriever,
        }
    }
}

impl<U> MdLinkAnalyzerIterSrc for MdLinkAnalyzer<U>
where
    U: ResourceIdRetriever + 'static + Clone,
{
    type Iter = std::vec::IntoIter<Link2Tgt>;

    fn iter(&self, content: Content) -> Self::Iter {
        // let ret: Vec<Link2Tgt> =
        //   extract_md_links(content.clone(), self.resource_id_retriever.clone()).collect();

        let ret: Vec<Link2Tgt> = arg_extract(
            extract_md_links,
            content,
            self.resource_id_retriever.clone(),
        )
        .collect();
        ret.into_iter()
    }
}
