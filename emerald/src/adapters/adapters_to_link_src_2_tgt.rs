use super::adapters_to_link_2_tgt::adapter_from_link_to_link_2_tgt;
use super::adapters_to_links::adapter_from_content_type_to_links;
use crate::{maps, markdown, types};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_analyze_md<'a, I: markdown::MarkdownAnalyzer<'a> + 'a + Copy>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::Content)> + 'a,
    md_analyzer: I,
) -> impl Iterator<Item = (&'a types::ResourceId, types::ContentType<'a>)> + 'a {
    it_src
        .into_iter()
        .flat_map(move |(rid, content)| md_analyzer.analyze(&content.0).map(move |f| (rid, f)))
}

pub fn adapter_from_rid_and_content_to_link_src_2_tgt<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::ContentType<'a>)> + 'a,
    rid_resolver: &'a impl maps::ResourceIdResolver,
) -> impl Iterator<Item = types::LinkSrc2Tgt> + 'a {
    let it1 = adapter_from_content_type_to_links(it_src.into_iter());
    let it2 = adapter_from_link_to_link_2_tgt(it1, rid_resolver);

    it2.into_iter()
        .map(|(rid, link_2_tgt)| types::LinkSrc2Tgt::from_link_to_target(rid.clone(), link_2_tgt))
}
