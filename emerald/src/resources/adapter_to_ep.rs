use crate::types::EndPoint;
use std::path::PathBuf;
use EndPoint::*;

pub fn adapter_from_pathes_to_ep(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = EndPoint> {
    let mut ep_list = Vec::<EndPoint>::new();
    for file_path in it_src {
        let endpoint = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            FileMarkdown(file_path)
        } else {
            FileUnknown(file_path)
        };

        ep_list.push(endpoint);
    }

    ep_list.into_iter()
}
