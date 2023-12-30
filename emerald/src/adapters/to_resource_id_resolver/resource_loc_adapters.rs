use crate::{model::resource_id_resolver::ResourceLoc, types, utils};
use std::path::Path;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Converts tuples of resource IDs and filesystem metadata into `ResourceLoc` instances.
///
/// This function processes each tuple in the provided iterator, transforming them into
/// `ResourceLoc` instances. It normalizes file names, calculates relative directory paths,
/// and pairs them with their respective resource IDs.
///
/// # Arguments
///
/// * `it_src` - An iterator over tuples, each containing a `types::ResourceId` and
///   `types::FilesystemMetadata`.
/// * `common_path` - A base path used to derive relative directory paths for the resources.
///
/// # Returns
///
/// An iterator of `ResourceLoc` instances, each representing the location and identifier
/// of a resource.
pub fn convert_to_resource_locations<'a>(
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

        // Attention: All normalized filennames are lowercase
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
