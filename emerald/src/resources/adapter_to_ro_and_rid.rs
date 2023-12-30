use super::{resource_object::ResourceObject, resource_object_translation::convert_ro_to_rid};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::path::Path;

pub fn adapter_to_ro_and_rid<'a>(
    it_src: impl IntoIterator<Item = &'a ResourceObject> + 'a,
    common_path: &'a Path,
) -> impl Iterator<Item = (&ResourceObject, types::ResourceId)> + 'a {
    it_src
        .into_iter()
        .map(move |ro| (ro, convert_ro_to_rid(ro, common_path)))
}
