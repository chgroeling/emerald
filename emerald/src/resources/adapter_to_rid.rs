use super::resource_object::ResourceObject;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_ep_to_rid<'a>(
    it_src: impl IntoIterator<Item = &'a (ResourceObject, types::ResourceId)> + 'a,
) -> impl Iterator<Item = types::ResourceId> + 'a {
    it_src.into_iter().map(|(_, rid)| rid.clone())
}
