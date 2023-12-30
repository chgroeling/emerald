use crate::{model::resource_id_resolver::ResourceLoc, types, utils};
use std::path::Path;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Converts resource IDs and filesystem metadata to `ResourceLoc`.
///
/// # Arguments
///
/// * `it_src` - Iterator of `(types::ResourceId, types::FilesystemMetadata)`.
/// * `common_path` - Base path for calculating relative paths.
///
/// # Returns
///
/// Iterator over `ResourceLoc`.
pub fn adapter_to_resourece_loc<'a>(
    it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)> + 'a,
    common_path: &'a Path,
) -> impl Iterator<Item = ResourceLoc> + 'a {
    it_src.into_iter().map(move |(rid, fs_metadata)| {
        let path_to_file = &fs_metadata.path;

        // get normalized name of file
        let os_filename = path_to_file
            .file_name()
            .expect("Pathes ending with '..' are not allowed here");
        let filename = os_filename
            .to_str()
            .expect("Filename must have a valid utf-8 representation");

        let norm_filename = utils::normalize_str(&filename.to_lowercase()).into_boxed_str();

        //
        let dir_path = path_to_file.parent().expect("Invalid directory path");
        let rel_dir_path = dir_path
            .strip_prefix(common_path)
            .expect("Common path is not part of path");
        let rel_dir_path = rel_dir_path
            .to_str()
            .expect("Directory path must have a valid utf-8 representation");

        // Replace all windows path chars
        let rel_dir_path: String = utils::normalize_str_iter(rel_dir_path)
            .map(|ch| match ch {
                '\\' => '/',
                _ => ch,
            })
            .collect();

        let rel_dir_path = rel_dir_path.into_boxed_str();

        trace!(
            "Insert {:?} -> ({:?}, {:?})",
            &norm_filename,
            &rid,
            &rel_dir_path
        );

        ResourceLoc {
            norm_filename,
            rid: rid.clone(),
            dir_path: rel_dir_path,
        }
    })
}
