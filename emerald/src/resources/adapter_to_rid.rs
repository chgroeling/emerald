use super::resource_object::ResourceObject;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_to_rid(
    it_src: impl IntoIterator<Item = (ResourceObject, types::ResourceId)>,
) -> impl Iterator<Item = types::ResourceId> {
    it_src.into_iter().map(|(_, rid)| rid)
}
