use crate::types::{EndPoint, ResourceId};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_ep_to_rid<'a>(
    it_src: impl IntoIterator<Item = &'a (EndPoint, ResourceId)> + 'a,
) -> impl Iterator<Item = ResourceId> + 'a {
    it_src.into_iter().map(|(_, rid)| rid.clone())
}
