use self::{
    convert_to_link_2_tgt::convert_to_link_2_tgt,
    convert_to_link_src_2_tgt::convert_to_link_src_2_tgt,
    extract_content_types::extract_content_types, extract_links::extract_links,
};
use crate::types::Content;
use crate::Result;
use crate::{
    maps::ResourceIdRetriever,
    resources::content_loader::ContentLoader,
    types::{LinkSrc2Tgt, ResourceId},
};
mod content_type;
mod convert_to_link_2_tgt;
mod convert_to_link_src_2_tgt;
mod extract_content_types;
mod extract_links;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;

pub fn extract_links_from_content<'a>(
    src: ResourceId,
    content: Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    trace!("Link extraction from {:?} starts", src);
    let content_type_iter = extract_content_types(content);
    let link_iter = extract_links(content_type_iter);
    let link_2_tgt_iter = convert_to_link_2_tgt(link_iter, resource_id_retriever);
    convert_to_link_src_2_tgt(src, link_2_tgt_iter)
}

pub fn extract_links_from_content_boxed<'a>(
    src: ResourceId,
    content: Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> LinkSrc2TgtIterBoxed<'a> {
    Box::new(extract_links_from_content(
        src,
        content,
        resource_id_retriever,
    ))
}

pub fn extract_links_from_vault<'a>(
    iter: impl Iterator<Item = ResourceId> + 'a,
    content_loader: &'a impl ContentLoader,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = (ResourceId, Result<LinkSrc2TgtIterBoxed<'a>>)> + 'a {
    // load content.
    // iterator yields (ResourceId, Result<Content>)
    let content_iter = iter.map(move |f| (f.clone(), content_loader.load(&f)));

    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    let all_links_iter = content_iter.map(move |f| {
        (
            f.0.clone(),
            f.1.map(move |content| {
                extract_links_from_content_boxed(f.0, content, resource_id_retriever)
            }),
        )
    });

    all_links_iter
}
