use super::resource_object::ResourceObject;
use crate::error::{EmeraldError::*, Result};
use crate::types;
use crate::utils;
use std::path::Path;

pub fn convert_ro_to_rid(ro: &ResourceObject, common_path: &Path) -> Result<types::ResourceId> {
    #[allow(clippy::infallible_destructuring_match)]
    let path = match ro {
        ResourceObject::File(path) => path,
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
    let path_iter = utils::normalize_str_iter(rel_path_str).map(|ch| match ch {
        '\\' => '/',
        _ => ch,
    });

    let res_id_str: String = path_iter.collect();
    Ok(res_id_str.into())
}

#[cfg(test)]
mod tests {
    use super::convert_ro_to_rid;
    use super::ResourceObject::*;
    use std::path::PathBuf;

    #[test]
    fn test_convert_unix_path_to_rid() {
        let common_path = PathBuf::from("");
        let ro = File("a/b/c/note.md".into());
        let link = convert_ro_to_rid(&ro, &common_path);
        assert_eq!(link.unwrap(), "a/b/c/note.md".into())
    }

    #[test]
    fn test_convert_windows_path_to_rid() {
        let common_path = PathBuf::from("");
        let ro = File("a\\b\\c\\note.md".into());
        let link = convert_ro_to_rid(&ro, &common_path);
        assert_eq!(link.unwrap(), "a/b/c/note.md".into())
    }
}
