use super::adapters_to_link_2_tgt::adapter_from_link_to_link_2_tgt;
use super::adapters_to_links::adapter_from_content_type_to_links;
use crate::markdown::MarkdownAnalyzer;
use crate::types::{Link2Tgt, LinkSrc2Tgt, ResourceId};
use crate::{maps::ResourceIdResolver, types::Content};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn adapter_from_link_2_tgt_to_link_src_2_tgt<'a>(
    it_src: impl IntoIterator<Item = Link2Tgt> + 'a,
    src: &'a ResourceId,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    it_src
        .into_iter()
        .map(move |f| LinkSrc2Tgt::from_link_to_target(src.clone(), f))
}

fn extract_links_from_content<'a, I: MarkdownAnalyzer<'a>>(
    src: &'a ResourceId,
    content: &'a Content,
    resource_id_retriever: &'a impl ResourceIdResolver,
    md_analyzer: I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    trace!("Link extraction from {:?} starts", src);
    let content_type_iter = md_analyzer.analyze(&content.0);
    let link_iter = adapter_from_content_type_to_links(content_type_iter);
    let link_2_tgt_iter = adapter_from_link_to_link_2_tgt(link_iter, resource_id_retriever);
    adapter_from_link_2_tgt_to_link_src_2_tgt(link_2_tgt_iter, src)
}

pub fn adapter_from_rid_and_content_to_link_src_2_tgt<'a, I: MarkdownAnalyzer<'a> + 'a + Copy>(
    it_src: impl IntoIterator<Item = (&'a ResourceId, &'a Content)> + 'a,
    resource_id_retriever: &'a impl ResourceIdResolver,
    md_analyzer: I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    it_src
        .into_iter()
        .flat_map(move |f| extract_links_from_content(f.0, f.1, resource_id_retriever, md_analyzer))
}
