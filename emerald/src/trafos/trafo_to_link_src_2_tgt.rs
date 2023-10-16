use std::vec;

use super::trafo_to_content_types::trafo_from_content_to_content_type;
use super::trafo_to_link_2_tgt::trafo_from_links_to_link_2_tgt;
use super::trafo_to_links::trafo_from_content_type_to_links;
use crate::types::{ContentType, Link2Tgt, LinkSrc2Tgt, ResourceId};
use crate::Result;
use crate::{maps::ResourceIdRetriever, types::Content};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;

fn trafo_from_link_2_tgt_to_link_src_2_tgt<'a>(
    src: &'a ResourceId,
    iter: impl Iterator<Item = Link2Tgt> + 'a,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    iter.map(move |f| LinkSrc2Tgt::from_link_to_target(src.clone(), f))
}

fn extract_links_from_content<'a, I, Iter>(
    src: &'a ResourceId,
    content: &'a Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a
where
    I: Fn(&'a str) -> Iter,
    Iter: Iterator<Item = ContentType<'a>> + 'a,
{
    trace!("Link extraction from {:?} starts", src);
    let content_type_iter = trafo_from_content_to_content_type(content, md_analyzer);
    let link_iter = trafo_from_content_type_to_links(content_type_iter);
    let link_2_tgt_iter = trafo_from_links_to_link_2_tgt(link_iter, resource_id_retriever);
    trafo_from_link_2_tgt_to_link_src_2_tgt(src, link_2_tgt_iter)
}

fn extract_links_from_content_boxed<'a, I, Iter>(
    src: &'a ResourceId,
    content: &'a Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> LinkSrc2TgtIterBoxed<'a>
where
    I: Fn(&'a str) -> Iter,
    Iter: Iterator<Item = ContentType<'a>> + 'a,
{
    Box::new(extract_links_from_content(
        src,
        content,
        resource_id_retriever,
        md_analyzer,
    ))
}

pub fn trafo_from_content_to_linksrc2tgt<'a, I, Iter>(
    iter: impl Iterator<Item = (&'a ResourceId, Result<&'a Content>)> + 'a,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> impl Iterator<Item = (ResourceId, Result<LinkSrc2TgtIterBoxed<'a>>)> + 'a
where
    I: Fn(&'a str) -> Iter,
    Iter: Iterator<Item = ContentType<'a>> + 'a,
{
    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    iter.map(move |f| {
        (
            f.0.clone(),
            f.1.map(move |content| {
                extract_links_from_content_boxed(f.0, content, resource_id_retriever, md_analyzer)
            }),
        )
    })
}

pub fn trafo_from_content_list_to_linksrc2tgt<'a, I, Iter>(
    iter: impl Iterator<Item = (&'a ResourceId, Result<&'a Content>)> + 'a,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a
where
    I: Fn(&'a str) -> Iter,
    Iter: Iterator<Item = ContentType<'a>> + 'a,
{
    let unwrap_iter = iter.filter_map(|f| {
        if let Ok(content) = f.1 {
            return Some((f.0, content));
        }
        None
    });
    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    unwrap_iter
        .flat_map(move |f| extract_links_from_content(f.0, f.1, resource_id_retriever, md_analyzer))
}
