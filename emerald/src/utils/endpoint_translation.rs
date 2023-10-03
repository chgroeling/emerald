use std::path::Path;

use super::normalize_string::normalize_str_iter;
use crate::types::EndPoint;
use crate::types::ResourceId;

const LINK_FRONT: &str = "[[";
const LINK_BACK: &str = "]]";

pub fn convert_endpoint_to_resource_id(
    endpoint: EndPoint,
    common_path: &Path,
) -> Option<ResourceId> {
    let path = match endpoint {
        EndPoint::File(path) => path,
        EndPoint::FileMarkdown(path) => path,
    };
    let rel_path = path.strip_prefix(common_path).unwrap().to_path_buf();

    // Replace all windows path chars
    let path_iter = normalize_str_iter(rel_path.to_str()?).map(|ch| match ch {
        '\\' => '/',
        _ => ch,
    });

    let res_id_iter = LINK_FRONT.chars().chain(path_iter.chain(LINK_BACK.chars()));
    let res_id_str: String = res_id_iter.collect();
    Some(res_id_str.into())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::convert_endpoint_to_resource_id;
    use super::EndPoint;
    use EndPoint::*;

    #[test]
    fn test_convert_unix_path_to_endpoint_link() {
        let common_path = PathBuf::from("");
        let endpoint = File("a/b/c/note.md".into());
        let link = convert_endpoint_to_resource_id(endpoint, &common_path);
        assert_eq!(link.unwrap(), "[[a/b/c/note.md]]".into())
    }

    #[test]
    fn test_convert_windows_path_to_endpoint_link() {
        let common_path = PathBuf::from("");
        let endpoint = File("a\\b\\c\\note.md".into());
        let link = convert_endpoint_to_resource_id(endpoint, &common_path);
        assert_eq!(link.unwrap(), "[[a/b/c/note.md]]".into())
    }
}
