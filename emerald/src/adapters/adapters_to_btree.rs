use super::adapters_to_link_2_tgt::adapter_to_link_2_tgt;
use super::adapters_to_links::adapter_to_rid_and_links;
use crate::{model::resource_id_resolver, types};

pub fn adapter_to_btree<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::MdBlock<'a>)> + 'a,
    rid_resolver: &'a impl resource_id_resolver::ResourceIdResolver,
) -> impl Iterator<Item = types::LinkSrc2Tgt> + 'a {
    let it1 = adapter_to_rid_and_links(it_src);
    let it2 = adapter_to_link_2_tgt(it1, rid_resolver);

    it2.into_iter()
        .map(|(rid, link_2_tgt)| types::LinkSrc2Tgt::from_link_to_target(rid.clone(), link_2_tgt))
}
