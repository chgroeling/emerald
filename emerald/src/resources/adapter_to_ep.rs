use crate::types;
use std::path::PathBuf;

pub fn adapter_from_pathes_to_ep(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = types::EndPoint> {
    let mut ep_list = Vec::<types::EndPoint>::new();
    for file_path in it_src {
        let endpoint = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            types::EndPoint::FileMarkdown(file_path)
        } else {
            types::EndPoint::FileUnknown(file_path)
        };

        ep_list.push(endpoint);
    }

    ep_list.into_iter()
}
