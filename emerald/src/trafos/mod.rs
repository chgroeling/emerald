use self::{
    convert_to_link_2_tgt::convert_to_link_2_tgt,
    convert_to_link_src_2_tgt::convert_to_link_src_2_tgt,
    extract_content_types::extract_content_types, extract_links::extract_links,
};
use crate::md_analyzer::ContentType;
use crate::types::Content;
use crate::Result;
use crate::{
    maps::ResourceIdRetriever,
    resources::content_loader::ContentLoader,
    types::{LinkSrc2Tgt, ResourceId},
};

mod convert_to_link_2_tgt;
mod convert_to_link_src_2_tgt;
mod extract_content_types;
mod extract_links;
mod trafo_to_filetype_and_resource_id;

pub use trafo_to_filetype_and_resource_id::trafo_to_filetype_and_resource_id;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;

pub fn extract_links_from_content<'a, I>(
    src: ResourceId,
    content: Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a
where
    I: Fn(&String) -> Vec<ContentType>,
{
    trace!("Link extraction from {:?} starts", src);
    let content_type_iter = extract_content_types(content, md_analyzer);
    let link_iter = extract_links(content_type_iter);
    let link_2_tgt_iter = convert_to_link_2_tgt(link_iter, resource_id_retriever);
    convert_to_link_src_2_tgt(src, link_2_tgt_iter)
}

pub fn extract_links_from_content_boxed<'a, I>(
    src: ResourceId,
    content: Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> LinkSrc2TgtIterBoxed<'a>
where
    I: Fn(&String) -> Vec<ContentType>,
{
    Box::new(extract_links_from_content(
        src,
        content,
        resource_id_retriever,
        md_analyzer,
    ))
}

pub fn extract_links_from_vault<'a, I>(
    iter: impl Iterator<Item = ResourceId> + 'a,
    content_loader: &'a impl ContentLoader,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: &'a I,
) -> impl Iterator<Item = (ResourceId, Result<LinkSrc2TgtIterBoxed<'a>>)> + 'a
where
    I: Fn(&String) -> Vec<ContentType>,
{
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
                extract_links_from_content_boxed(f.0, content, resource_id_retriever, md_analyzer)
            }),
        )
    });

    all_links_iter
}
