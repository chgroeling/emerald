use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_to_rid_and_links<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::MdBlock<'a>)> + 'a,
) -> impl Iterator<Item = (&'a types::ResourceId, types::Link)> + 'a {
    it_src
        .into_iter()
        .filter(|(_, content_type)| matches!(content_type, types::MdBlock::WikiLink(_)))
        .map(|(rid, content_type)| match content_type {
            types::MdBlock::WikiLink(link) => (rid, types::Link(link.to_owned())),
            _ => panic!(),
        })
}
