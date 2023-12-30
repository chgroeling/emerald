use super::resource_object::ResourceObject;
use crate::types;
use std::path::Path;

pub fn convert_ro_to_rid(ro: &ResourceObject, common_path: &Path) -> types::ResourceId {
    #[allow(clippy::infallible_destructuring_match)]
    let path = match ro {
        ResourceObject::File(path) => path,
    };

    let rel_path = path
        .strip_prefix(common_path)
        .expect("Common path is not part of path");
    let rel_path_str = rel_path
        .to_str()
        .expect("Directory path must have a valid utf-8 representation");

    rel_path_str.into()
}

