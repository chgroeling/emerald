use super::adapters_to_link_2_tgt::adapter_from_link_to_link_2_tgt;
use super::trafo_to_links::trafo_from_content_type_to_links;

use crate::markdown::MarkdownAnalyzer;
use crate::types::{Link2Tgt, LinkSrc2Tgt, ResourceId};
use crate::Result;
use crate::{maps::ResourceIdRetriever, types::Content};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn trafo_from_link_2_tgt_to_link_src_2_tgt<'a>(
    src: &'a ResourceId,
    it_src: impl IntoIterator<Item = Link2Tgt> + 'a,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    it_src
        .into_iter()
        .map(move |f| LinkSrc2Tgt::from_link_to_target(src.clone(), f))
}

fn extract_links_from_content<'a, I: MarkdownAnalyzer<'a>>(
    src: &'a ResourceId,
    content: &'a Content,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    trace!("Link extraction from {:?} starts", src);
    let content_type_iter = md_analyzer.analyze(&content.0);
    let link_iter = trafo_from_content_type_to_links(content_type_iter);
    let link_2_tgt_iter = adapter_from_link_to_link_2_tgt(link_iter, resource_id_retriever);
    trafo_from_link_2_tgt_to_link_src_2_tgt(src, link_2_tgt_iter)
}

pub fn trafo_from_content_list_to_linksrc2tgt<'a, I: MarkdownAnalyzer<'a> + 'a + Copy>(
    it_src: impl IntoIterator<Item = (&'a ResourceId, Result<&'a Content>)> + 'a,
    resource_id_retriever: &'a impl ResourceIdRetriever,
    md_analyzer: I,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    let uwit = it_src.into_iter().filter_map(|f| {
        if let Ok(content) = f.1 {
            return Some((f.0, content));
        }
        None
    });
    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    uwit.flat_map(move |f| extract_links_from_content(f.0, f.1, resource_id_retriever, md_analyzer))
}
