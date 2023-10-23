use super::resource_object::ResourceObject;
use std::path::PathBuf;

pub fn adapter_from_pathes_to_ro(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = ResourceObject> {
    let mut ro_list = Vec::<ResourceObject>::new();
    for file_path in it_src {
        ro_list.push(ResourceObject::File(file_path));
    }

    ro_list.into_iter()
}
