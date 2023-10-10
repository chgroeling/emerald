use self::{
    convert_to_link_2_tgt::convert_to_link_2_tgt,
    convert_to_link_src_2_tgt::convert_to_link_src_2_tgt,
    extract_content_types::extract_content_types, extract_links::extract_links,
};
use crate::Result;
use crate::{
    maps::ResourceIdRetriever,
    resources::content_loader::ContentLoader,
    types::{Content, Link2Tgt, LinkSrc2Tgt, ResourceId},
};
mod content_type;
mod convert_to_link_2_tgt;
mod convert_to_link_src_2_tgt;
mod extract_content_types;
mod extract_links;

pub fn extract_link2tgt<'a>(
    content: Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> + 'a {
    let content_type_iter = extract_content_types(content);
    let link_iter = extract_links(content_type_iter);
    convert_to_link_2_tgt(link_iter, resource_id_retriever)
}

pub fn extract_linksrc2tgt<'a>(
    src: ResourceId,
    content_loader: &'a impl ContentLoader,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> Result<impl Iterator<Item = LinkSrc2Tgt> + 'a> {
    let content = content_loader.load(&src)?;
    let link_2_tgt_iter = extract_link2tgt(content, resource_id_retriever);
    Ok(convert_to_link_src_2_tgt(src, link_2_tgt_iter))
}
