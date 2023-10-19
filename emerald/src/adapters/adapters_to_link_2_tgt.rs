use crate::{
    maps::ResourceIdRetriever,
    types::{Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_from_link_to_link_2_tgt<'a>(
    it_src: impl IntoIterator<Item = Link> + 'a,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> + 'a {
    it_src.into_iter().map(|f| {
        if let Ok(resource_id) = resource_id_retriever.retrieve(&f) {
            Link2Tgt::new(f, Some(resource_id))
        } else {
            Link2Tgt::new(f, None)
        }
    })
}
