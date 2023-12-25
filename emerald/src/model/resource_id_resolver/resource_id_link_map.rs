use super::resource_id_resolver_trait::Hint;
use super::resource_id_resolver_trait::ResourceIdResolver;
use crate::error::{EmeraldError::*, Result};
use crate::{types, utils};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub type NameToResourceIdList = HashMap<String, Vec<(types::ResourceId, String)>>;

#[derive(Clone)]
pub struct ResourceIdLinkMap {
    name_to_rid_list: NameToResourceIdList,
}

impl ResourceIdLinkMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)>,
        common_path: &'a Path,
    ) -> Self {
        // Assumption: All resource ids are encoded in utf8 nfc
        let mut name_to_rid_list: NameToResourceIdList = NameToResourceIdList::new();

        for (rid, fs_metadata) in it_src.into_iter() {
            let path_to_file = fs_metadata.path.clone();

            // TODO: INTRODUCE RESULT TYPE
            // get name of file
            let os_filename = path_to_file.file_name().unwrap();
            let filename = os_filename.to_str().unwrap().to_string().to_lowercase();
            let normalized_link = utils::normalize_str(&filename);
            //
            let path = path_to_file.parent().unwrap();
            let path_2 = path.strip_prefix(common_path).unwrap();
            let path_str = path_2.to_str().unwrap();
            // Replace all windows path chars
            let path_str: String = utils::normalize_str_iter(path_str)
                .map(|ch| match ch {
                    '\\' => '/',
                    _ => ch,
                })
                .collect();

            trace!(
                "Insert {:?} -> ({:?}, {:?})",
                &normalized_link,
                &rid,
                &path_str
            );

            // this is an interesting way to mutate an element in a HashMap
            match name_to_rid_list.entry(normalized_link) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push((rid.clone(), path_str));
                }
                Entry::Vacant(e) => {
                    e.insert(vec![(rid.clone(), path_str)]);
                }
            }
        }

        ResourceIdLinkMap { name_to_rid_list }
    }
}

impl ResourceIdResolver for ResourceIdLinkMap {
    fn resolve_with_hint(&self, link: &types::Link, _hint: Hint) -> Result<types::ResourceId> {
        // convert string to internal link format
        let link_comp = link.split()?;
        let link_name_lc = utils::normalize_str(&link_comp.name.trim().to_lowercase());

        // check if md files in our hashmap are matching the given link
        let matches_of_exact_name = self.name_to_rid_list.get(&link_name_lc);

        // no .. then perhaps there are files without adding ".md" that will match
        let matches = if matches_of_exact_name.is_none() {
            // add a .md extension to the link to check if a note with this name exists
            let link_name_lc_md = link_name_lc.clone() + ".md";

            self.name_to_rid_list.get(&link_name_lc_md)
        } else {
            matches_of_exact_name
        };

        // ok the link matched
        if let Some(match_list) = matches {
            assert!(!match_list.is_empty());
            trace!(
                "Name of link {} found in index. Resulting match_list: {:?}",
                &link_comp,
                &match_list
            );

            // Check if the given link has a path
            if let Some(link_path) = &link_comp.path {
                let link_path_norm = utils::normalize_str(link_path);

                // if it has one ... try to match it with the result list.
                for (rid, plink_path) in match_list {
                    // Assumption: plink_path is already utf8 nfc encoded
                    if plink_path == &link_path_norm {
                        return Ok(rid.clone());
                    }
                }
                // no link found
            } else {
                // not path was specified
                if match_list.len() > 1 {
                    warn!("The link {} is not unique.", &link_comp);
                }

                let match_link = match_list[0].clone();
                return Ok(match_link.0);
            }
        }

        trace!("find_link - No link found - \"{}\"", &link_name_lc);
        Err(LinkNotFound(link_comp.to_string()))
    }
}

#[cfg(test)]
mod link_mapper_tests {
    /*
    use super::*;
    use crate::types::ResourceId;
    use std::iter::zip;

    fn create_dut(res_ids: Vec<ResourceId>, names: Vec<String>) -> ResourceIdLinkMap {
        let iter = zip(res_ids.into_iter(), names);
        ResourceIdLinkMap::new(iter)
    }

    #[test]
    fn test_malformed_link_causes_error() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[note1]]".into()).unwrap_err();
        assert!(matches!(result, NotAWikiLink));
    }

    #[test]
    fn test_link_match_without_extension() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[[note1]]".into()).unwrap();
        assert_eq!(result, "[[note1.md]]".into());
    }

    #[test]
    fn test_link_match_without_extension_with_spaces() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[[note1  ]]".into()).unwrap();
        assert_eq!(result, "[[note1.md]]".into());
    }

    #[test]
    fn test_link_match_without_extension_and_double_dot() {
        let dut = create_dut(vec!["[[note1..md]]".into()], vec!["note1..md".to_string()]);
        let result = dut.resolve(&"[[note1.]]".into()).unwrap();
        assert_eq!(result, "[[note1..md]]".into());
    }

    #[test]
    fn test_link_miss_without_extension_and_double_dot() {
        let dut = create_dut(vec!["[[note1..md]]".into()], vec!["note1..md".to_string()]);
        let result = dut.resolve(&"[[note1]]".into()).unwrap_err();
        assert!(matches!(result, LinkNotFound(failed_link) if failed_link == "[[note1]]"));
    }

    #[test]
    fn test_link_match_with_extension() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[[note1.md]]".into()).unwrap();
        assert_eq!(result, "[[note1.md]]".into());
    }

    #[test]
    fn test_link_miss_without_extension() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[[missing]]".into()).unwrap_err();
        assert!(matches!(result, LinkNotFound(failed_link) if failed_link == "[[missing]]"));
    }

    #[test]
    fn test_link_miss_with_extension() {
        let dut = create_dut(vec!["[[note1.md]]".into()], vec!["note1.md".to_string()]);
        let result = dut.resolve(&"[[missing.md]]".into()).unwrap_err();
        assert!(matches!(result, LinkNotFound(failed_link) if failed_link == "[[missing.md]]"));
    }

    #[test]
    fn test_link_match_two_files_at_different_pathes() {
        let dut = create_dut(
            vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()],
            vec!["note1.md".to_string(), "note1.md".to_string()],
        );
        let result = dut.resolve(&"[[note1]]".into());
        assert_eq!(result.unwrap(), "[[path1/note1.md]]".into());
    }

    #[test]
    fn test_link_match_two_files_same_name_different_ext() {
        let dut = create_dut(
            vec!["[[path1/note1]]".into(), "[[path2/note1.md]]".into()],
            vec!["note1".to_string(), "note1.md".to_string()],
        );
        let result = dut.resolve(&"[[note1]]".into()).unwrap();

        // always return the exact match even when a md file exists.
        assert_eq!(result, "[[path1/note1]]".into());
    }

    #[test]
    fn test_absolute_link_match_two_files_at_different_pathes() {
        let dut = create_dut(
            vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()],
            vec!["note1.md".to_string(), "note1.md".to_string()],
        );
        let result = dut.resolve(&"[[path2/note1]]".into());
        assert_eq!(result.unwrap(), "[[path2/note1.md]]".into());
    }

    #[test]
    fn test_absolute_link_match_two_files_at_different_pathes_with_extension() {
        let dut = create_dut(
            vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()],
            vec!["note1.md".to_string(), "note1.md".to_string()],
        );
        let result = dut.resolve(&"[[path2/note1.md]]".into()).unwrap();
        assert_eq!(result, "[[path2/note1.md]]".into());
    }

    #[test]
    fn test_resolve_endpoint_link_path_has_different_utf8_representation() {
        let dut = create_dut(
            vec!["[[päth1/note1.md]]".into(), "[[päth2/note1.md]]".into()],
            vec!["note1.md".to_string(), "note1.md".to_string()],
        );
        // Attention: The "ä" from above is coded differently than the following ä
        let result = dut.resolve(&"[[päth2/note1.md]]".into()).unwrap();
        assert_eq!(result, "[[päth2/note1.md]]".into());
    }

    #[test]
    fn test_resolve_endpoint_link_name_has_different_utf8_representation() {
        let dut = create_dut(
            vec!["[[path1/nöte1.md]]".into(), "[[path2/nöte1.md]]".into()],
            vec!["nöte1.md".to_string(), "nöte1.md".to_string()],
        );
        // Attention: The "ö" from above is coded differently than the following ö
        let result = dut.resolve(&"[[path2/nöte1.md]]".into()).unwrap();
        assert_eq!(result, "[[path2/nöte1.md]]".into());
    }

    */
}
