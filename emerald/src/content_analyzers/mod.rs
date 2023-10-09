use crate::{
    maps::ResourceIdRetriever,
    types::{Content, Link2Tgt},
};

use self::{
    link_extractor::extract_links, md_extractor::extract_content_types,
    resource_id_extractor::convert_to_resource_id,
};
mod content_type;
mod link_extractor;
mod md_extractor;
mod resource_id_extractor;

pub fn extract_md_links(
    content: Content,
    resource_id_retriever: impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> {
    let content_type_iter = extract_content_types(content);
    let link_iter = extract_links(content_type_iter);
    convert_to_resource_id(link_iter, resource_id_retriever)
}

pub type AbstractLinkExtractor<T, C> = fn(Content, T) -> C;
