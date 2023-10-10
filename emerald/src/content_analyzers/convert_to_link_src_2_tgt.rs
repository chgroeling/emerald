use crate::{
    maps::ResourceIdRetriever,
    types::{Link, Link2Tgt, LinkSrc2Tgt, ResourceId},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn convert_to_link_src_2_tgt<'a>(
    src: ResourceId,
    iter: impl Iterator<Item = Link2Tgt> + 'a,
) -> impl Iterator<Item = LinkSrc2Tgt> + 'a {
    iter.map(move |f| LinkSrc2Tgt::from_link_to_target(src.clone(), f))
}
