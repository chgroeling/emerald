use crate::{
    maps::ResourceIdRetriever,
    types::{Link, Link2Tgt},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn trafo_from_links_to_link_2_tgt<'a>(
    iter: impl Iterator<Item = Link> + 'a,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = Link2Tgt> + 'a {
    iter.map(|f| {
        if let Ok(resource_id) = resource_id_retriever.retrieve(&f) {
            Link2Tgt::new(f, Some(resource_id))
        } else {
            Link2Tgt::new(f, None)
        }
    })
}
