use crate::types::EndPoint;
use std::path::PathBuf;
use EndPoint::*;

pub fn adapter_from_pathes_to_endpoints(
    it_src: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = EndPoint> {
    let mut endpoint_list = Vec::<EndPoint>::new();
    for file_path in it_src {
        let endpoint = if file_path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            FileMarkdown(file_path)
        } else {
            FileUnknown(file_path)
        };

        endpoint_list.push(endpoint);
    }

    endpoint_list.into_iter()
}
