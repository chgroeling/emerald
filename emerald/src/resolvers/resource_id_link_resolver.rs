use crate::indexes::AllResourceIdsIterSource;
use crate::types::link::Link;
use crate::types::link_decomposer::LinkDecomposer;
use crate::types::ResourceId;
use crate::utils::normalize_string::normalize_str;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::link_resolver::Hint;
use super::link_resolver::LinkResolver;

pub type ResourceIdList = Vec<ResourceId>;
pub type NameToResourceIdList = HashMap<String, ResourceIdList>;

pub struct ResourceIdLinkResolver {
    link_decomposer: LinkDecomposer,
    name_to_resource_id_list: NameToResourceIdList,
}

impl ResourceIdLinkResolver {
    // Assumption: All EndPoints are coded utf8 nfc
    pub fn new(ep_iter_src: &impl AllResourceIdsIterSource) -> Self {
        let mut name_to_resource_id_list: NameToResourceIdList = NameToResourceIdList::new();
        let link_decomposer = LinkDecomposer::new();

        // Iterator yields (normalized_link, link_to_file)
        let link_name_iter = ep_iter_src.all_iter().map(|resource_id| {
            let dc_link = link_decomposer.decompose(&resource_id.0).unwrap();
            let normalized_link = dc_link.link.to_lowercase();

            (normalized_link, resource_id)
        });

        for (normalized_link, resource_id) in link_name_iter {
            trace!("Insert {:?} -> {}", normalized_link, &resource_id.0);

            // this is an interesting way to mutate an element in a HashMap
            match name_to_resource_id_list.entry(normalized_link) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(resource_id);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![resource_id]);
                }
            }
        }

        ResourceIdLinkResolver {
            name_to_resource_id_list,
            link_decomposer,
        }
    }
}

impl LinkResolver for ResourceIdLinkResolver {
    fn get_with_hint(&self, link: &Link, _hint: Hint) -> Result<ResourceId> {
        // convert string to internal link format
        let dec_link = self.link_decomposer.decompose(&link.0)?;
        let link_name_lc = normalize_str(&dec_link.link.trim().to_lowercase());

        // check if md files in our hashmap are matching the given link
        let matches_of_exact_name = self.name_to_resource_id_list.get(&link_name_lc);

        // no .. then perhaps there are files without adding ".md" that will match
        let matches: Option<_> = if matches_of_exact_name.is_none() {
            // add a .md extension to the link to check if a note with this name exists
            let link_name_lc_md = link_name_lc.clone() + ".md";

            self.name_to_resource_id_list.get(&link_name_lc_md)
        } else {
            matches_of_exact_name
        };

        // ok the link matched
        if let Some(match_list) = matches {
            assert!(!match_list.is_empty());
            trace!(
                "Name of link {} found in index. Resulting match_list: {:?}",
                &dec_link,
                &match_list
            );

            // Check if the given link has a path
            if let Some(link_path) = &dec_link.path {
                let link_path_norm = normalize_str(&link_path);

                // if it has one ... try to match it with the result list.
                for potential_link in match_list {
                    let de_potential_link = self.link_decomposer.decompose(&potential_link.0)?;

                    if let Some(plink_path) = de_potential_link.path {
                        // Assumption: plink_path is already utf8 nfc encoded
                        if plink_path == link_path_norm {
                            return Ok(potential_link.clone());
                        }
                    }
                }
                // no link found
            } else {
                // not path was specified
                if match_list.len() > 1 {
                    warn!("The link {} is not unique.", &dec_link);
                }

                let match_link = match_list[0].clone();
                return Ok(match_link);
            }
        }

        trace!("find_link - No link found - \"{}\"", &link_name_lc);
        Err(LinkNotFound(dec_link.to_string()))
    }
}

#[cfg(test)]
mod link_mapper_tests {
    use super::AllResourceIdsIterSource;
    use super::EmeraldError::*;
    use super::LinkResolver;
    use super::ResourceId;
    use super::ResourceIdLinkResolver;

    struct MockFileIndex {
        links: Vec<ResourceId>,
    }

    impl AllResourceIdsIterSource for MockFileIndex {
        type Iter = std::vec::IntoIter<ResourceId>;

        fn all_iter(&self) -> Self::Iter {
            self.links.clone().into_iter()
        }
    }

    fn prepare_mock_file_index(link_list: Vec<ResourceId>) -> MockFileIndex {
        MockFileIndex { links: link_list }
    }

    #[test]
    fn check_malformed_link_causes_error() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[note1]]".into());

        assert!(result.is_err_and(|f| matches!(f, NotAWikiLink)));
    }

    #[test]
    fn check_link_match_without_extension() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_match_without_extension_with_spaces() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1  ]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_match_without_extension_and_double_dot() {
        let file_index = prepare_mock_file_index(vec!["[[note1..md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1.]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1..md]]".into()));
    }

    #[test]
    fn check_link_miss_without_extension_and_double_dot() {
        let file_index = prepare_mock_file_index(vec!["[[note1..md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1]]".into());

        assert!(result
            .is_err_and(|f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[note1]]")));
    }
    #[test]
    fn check_link_match_with_extension() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1.md]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_miss_without_extension() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[missing]]".into());

        assert!(result.is_err_and(
            |f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[missing]]")
        ));
    }

    #[test]
    fn check_link_miss_with_extension() {
        let file_index = prepare_mock_file_index(vec!["[[note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[missing.md]]".into());

        assert!(result.is_err_and(
            |f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[missing.md]]")
        ));
    }

    #[test]
    fn check_link_match_two_files_at_different_pathes() {
        let file_index = prepare_mock_file_index(vec![
            "[[path1/note1.md]]".into(),
            "[[path2/note1.md]]".into(),
        ]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1]]".into());

        assert!(result.is_ok_and(|f| f == "[[path1/note1.md]]".into()));
    }

    #[test]
    fn check_link_match_two_files_same_name_different_ext() {
        let file_index =
            prepare_mock_file_index(vec!["[[path1/note1]]".into(), "[[path2/note1.md]]".into()]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[note1]]".into());

        // always return the exact match even when a md file exists.
        assert!(result.is_ok_and(|f| f == "[[path1/note1]]".into()));
    }

    #[test]
    fn check_absolute_link_match_two_files_at_different_pathes() {
        let file_index = prepare_mock_file_index(vec![
            "[[path1/note1.md]]".into(),
            "[[path2/note1.md]]".into(),
        ]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[path2/note1]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/note1.md]]".into()));
    }

    #[test]
    fn check_absolute_link_match_two_files_at_different_pathes_with_extension() {
        let file_index = prepare_mock_file_index(vec![
            "[[path1/note1.md]]".into(),
            "[[path2/note1.md]]".into(),
        ]);
        let dut = ResourceIdLinkResolver::new(&file_index);

        let result = dut.get(&"[[path2/note1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/note1.md]]".into()));
    }

    #[test]
    fn check_resolve_endpoint_link_path_has_different_utf8_representation() {
        let file_index = prepare_mock_file_index(vec![
            "[[päth1/note1.md]]".into(),
            "[[päth2/note1.md]]".into(),
        ]);

        let dut = ResourceIdLinkResolver::new(&file_index);

        // Attention: The "ä" from above is coded differently than the following ä
        let result = dut.get(&"[[päth2/note1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[päth2/note1.md]]".into()));
    }

    #[test]
    fn check_resolve_endpoint_link_name_has_different_utf8_representation() {
        let file_index = prepare_mock_file_index(vec![
            "[[path1/nöte1.md]]".into(),
            "[[path2/nöte1.md]]".into(),
        ]);

        let dut = ResourceIdLinkResolver::new(&file_index);

        // Attention: The "ö" from above is coded differently than the following ö
        let result = dut.get(&"[[path2/nöte1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/nöte1.md]]".into()));
    }
}
