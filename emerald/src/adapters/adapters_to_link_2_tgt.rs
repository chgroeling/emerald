use crate::{model, types};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_from_link_to_link_2_tgt<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::Link)> + 'a,
    rid_resolver: &'a impl model::ResourceIdResolver,
) -> impl Iterator<Item = (&'a types::ResourceId, types::Link2Tgt)> + 'a {
    it_src.into_iter().map(|(rid, f)| {
        if let Ok(tgt_rid) = rid_resolver.resolve(&f) {
            (rid, types::Link2Tgt::new(f, Some(tgt_rid)))
        } else {
            (rid, types::Link2Tgt::new(f, None))
        }
    })
}
