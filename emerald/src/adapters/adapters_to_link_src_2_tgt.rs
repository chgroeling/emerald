use super::adapters_to_link_2_tgt::adapter_from_link_to_link_2_tgt;
use super::adapters_to_links::adapter_from_content_type_to_links;
use crate::{model::link_model, types};

pub fn adapter_to_link_src_2_tgt<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::ContentType<'a>)> + 'a,
    rid_resolver: &'a impl link_model::ResourceIdResolver,
) -> impl Iterator<Item = types::LinkSrc2Tgt> + 'a {
    let it1 = adapter_from_content_type_to_links(it_src);
    let it2 = adapter_from_link_to_link_2_tgt(it1, rid_resolver);

    it2.into_iter()
        .map(|(rid, link_2_tgt)| types::LinkSrc2Tgt::from_link_to_target(rid.clone(), link_2_tgt))
}
