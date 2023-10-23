use crate::types;
use std::path::PathBuf;

pub fn adapter_from_pathes_to_ep(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = types::ResourceObject> {
    let mut ep_list = Vec::<types::ResourceObject>::new();
    for file_path in it_src {
        let endpoint = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            types::ResourceObject::FileMarkdown(file_path)
        } else {
            types::ResourceObject::FileUnknown(file_path)
        };

        ep_list.push(endpoint);
    }

    ep_list.into_iter()
}
