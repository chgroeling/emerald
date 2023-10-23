use crate::{maps, types};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_from_link_to_link_2_tgt<'a>(
    it_src: impl IntoIterator<Item = types::Link> + 'a,
    rid_resolver: &'a impl maps::ResourceIdResolver,
) -> impl Iterator<Item = types::Link2Tgt> + 'a {
    it_src.into_iter().map(|f| {
        if let Ok(rid) = rid_resolver.resolve(&f) {
            types::Link2Tgt::new(f, Some(rid))
        } else {
            types::Link2Tgt::new(f, None)
        }
    })
}
