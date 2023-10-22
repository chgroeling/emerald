use super::normalize_string::normalize_str_iter;
use crate::types;
use std::path::Path;
use types::{EmeraldError::*, Result};

const LINK_FRONT: &str = "[[";
const LINK_BACK: &str = "]]";

pub fn convert_endpoint_to_resource_id(
    endpoint: &types::EndPoint,
    common_path: &Path,
) -> Result<types::ResourceId> {
    let path = match endpoint {
        types::EndPoint::FileUnknown(path) => path,
        types::EndPoint::FileMarkdown(path) => path,
    };
    let rel_path = match path.strip_prefix(common_path) {
        Ok(item) => item.to_path_buf(),
        Err(_) => return Err(ValueError),
    };

    let rel_path_str = match rel_path.to_str() {
        Some(res) => res,
        None => return Err(ValueError),
    };

    // Replace all windows path chars
    let path_iter = normalize_str_iter(rel_path_str).map(|ch| match ch {
        '\\' => '/',
        _ => ch,
    });

    let res_id_iter = LINK_FRONT.chars().chain(path_iter.chain(LINK_BACK.chars()));
    let res_id_str: String = res_id_iter.collect();
    Ok(res_id_str.into())
}

#[cfg(test)]
mod tests {
    use super::convert_endpoint_to_resource_id;
    use super::types;
    use std::path::PathBuf;
    use types::EndPoint::*;

    #[test]
    fn test_convert_unix_path_to_endpoint_link() {
        let common_path = PathBuf::from("");
        let endpoint = FileUnknown("a/b/c/note.md".into());
        let link = convert_endpoint_to_resource_id(&endpoint, &common_path);
        assert_eq!(link.unwrap(), "[[a/b/c/note.md]]".into())
    }

    #[test]
    fn test_convert_windows_path_to_endpoint_link() {
        let common_path = PathBuf::from("");
        let endpoint = FileUnknown("a\\b\\c\\note.md".into());
        let link = convert_endpoint_to_resource_id(&endpoint, &common_path);
        assert_eq!(link.unwrap(), "[[a/b/c/note.md]]".into())
    }
}
