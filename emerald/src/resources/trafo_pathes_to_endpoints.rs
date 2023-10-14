use crate::types::EndPoint;
use std::path::PathBuf;
use EndPoint::*;

pub fn trafo_pathes_to_endpoints(file_list: Vec<PathBuf>) -> impl Iterator<Item = EndPoint> {
    let mut endpoint_list = Vec::<EndPoint>::new();
    for file_path in file_list {
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
