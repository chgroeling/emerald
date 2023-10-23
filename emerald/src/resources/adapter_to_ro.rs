use super::resource_object::ResourceObject;
use std::path::PathBuf;

pub fn adapter_from_pathes_to_ro(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = ResourceObject> {
    let mut ro_list = Vec::<ResourceObject>::new();
    for file_path in it_src {
        let ro = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            ResourceObject::FileMarkdown(file_path)
        } else {
            ResourceObject::FileUnknown(file_path)
        };

        ro_list.push(ro);
    }

    ro_list.into_iter()
}
