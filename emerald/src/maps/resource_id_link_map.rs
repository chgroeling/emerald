use crate::types::link::Link;
use crate::types::ResourceId;
use crate::utils::normalize_string::normalize_str;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use crate::EmeraldError;
use crate::Result;

use EmeraldError::*;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_id_retriever::Hint;
use super::resource_id_retriever::ResourceIdRetriever;

pub type NameToResourceIdList = HashMap<String, Vec<ResourceId>>;

#[derive(Clone)]
pub struct ResourceIdLinkMap {
    name_to_resource_id_list: Rc<NameToResourceIdList>,
}

impl ResourceIdLinkMap {
    pub fn new<'a>(resource_ids_iter: impl Iterator<Item = &'a ResourceId>) -> Self {
        // Assumption: All resource ids are encoded in utf8 nfc
        let mut name_to_resource_id_list: NameToResourceIdList = NameToResourceIdList::new();

        // Iterator yields (normalized_link, link_to_file)
        let link_name_iter = resource_ids_iter.map(|resource_id| {
            let res_id_comp = resource_id.split().unwrap();
            let normalized_link = res_id_comp.name.to_lowercase();

            (normalized_link, resource_id.clone())
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

        ResourceIdLinkMap {
            name_to_resource_id_list: Rc::new(name_to_resource_id_list),
        }
    }
}

impl ResourceIdRetriever for ResourceIdLinkMap {
    fn retrieve_with_hint(&self, link: &Link, _hint: Hint) -> Result<ResourceId> {
        // convert string to internal link format
        let link_comp = link.split()?;
        let link_name_lc = normalize_str(&link_comp.name.trim().to_lowercase());

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
                &link_comp,
                &match_list
            );

            // Check if the given link has a path
            if let Some(link_path) = &link_comp.path {
                let link_path_norm = normalize_str(link_path);

                // if it has one ... try to match it with the result list.
                for potential_link in match_list {
                    let de_potential_link = potential_link.split()?;

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
                    warn!("The link {} is not unique.", &link_comp);
                }

                let match_link = match_list[0].clone();
                return Ok(match_link);
            }
        }

        trace!("find_link - No link found - \"{}\"", &link_name_lc);
        Err(LinkNotFound(link_comp.to_string()))
    }
}

#[cfg(test)]
mod link_mapper_tests {
    use crate::indexes::ResourceIdsIterSrc;

    use super::EmeraldError::*;
    use super::ResourceId;
    use super::ResourceIdLinkMap;
    use super::ResourceIdRetriever;

    struct MockFileIndex {
        links: Vec<ResourceId>,
    }

    impl ResourceIdsIterSrc for MockFileIndex {
        type Iter = std::vec::IntoIter<ResourceId>;

        fn iter(&self) -> Self::Iter {
            self.links.clone().into_iter()
        }
    }

    #[test]
    fn check_malformed_link_causes_error() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[note1]]".into());

        assert!(result.is_err_and(|f| matches!(f, NotAWikiLink)));
    }

    #[test]
    fn check_link_match_without_extension() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_match_without_extension_with_spaces() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1  ]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_match_without_extension_and_double_dot() {
        let file_index = vec!["[[note1..md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1.]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1..md]]".into()));
    }

    #[test]
    fn check_link_miss_without_extension_and_double_dot() {
        let file_index = vec!["[[note1..md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1]]".into());

        assert!(result
            .is_err_and(|f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[note1]]")));
    }
    #[test]
    fn check_link_match_with_extension() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1.md]]".into());

        assert!(result.is_ok_and(|f| f == "[[note1.md]]".into()));
    }

    #[test]
    fn check_link_miss_without_extension() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[missing]]".into());

        assert!(result.is_err_and(
            |f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[missing]]")
        ));
    }

    #[test]
    fn check_link_miss_with_extension() {
        let file_index = vec!["[[note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[missing.md]]".into());

        assert!(result.is_err_and(
            |f| matches!(f, LinkNotFound(failed_link) if failed_link == "[[missing.md]]")
        ));
    }

    #[test]
    fn check_link_match_two_files_at_different_pathes() {
        let file_index = vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1]]".into());

        assert!(result.is_ok_and(|f| f == "[[path1/note1.md]]".into()));
    }

    #[test]
    fn check_link_match_two_files_same_name_different_ext() {
        let file_index = vec!["[[path1/note1]]".into(), "[[path2/note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[note1]]".into());

        // always return the exact match even when a md file exists.
        assert!(result.is_ok_and(|f| f == "[[path1/note1]]".into()));
    }

    #[test]
    fn check_absolute_link_match_two_files_at_different_pathes() {
        let file_index = vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[path2/note1]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/note1.md]]".into()));
    }

    #[test]
    fn check_absolute_link_match_two_files_at_different_pathes_with_extension() {
        let file_index = vec!["[[path1/note1.md]]".into(), "[[path2/note1.md]]".into()];
        let dut = ResourceIdLinkMap::new(file_index.iter());

        let result = dut.retrieve(&"[[path2/note1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/note1.md]]".into()));
    }

    #[test]
    fn check_resolve_endpoint_link_path_has_different_utf8_representation() {
        let file_index = vec!["[[päth1/note1.md]]".into(), "[[päth2/note1.md]]".into()];

        let dut = ResourceIdLinkMap::new(file_index.iter());

        // Attention: The "ä" from above is coded differently than the following ä
        let result = dut.retrieve(&"[[päth2/note1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[päth2/note1.md]]".into()));
    }

    #[test]
    fn check_resolve_endpoint_link_name_has_different_utf8_representation() {
        let file_index = vec!["[[path1/nöte1.md]]".into(), "[[path2/nöte1.md]]".into()];

        let dut = ResourceIdLinkMap::new(file_index.iter());

        // Attention: The "ö" from above is coded differently than the following ö
        let result = dut.retrieve(&"[[path2/nöte1.md]]".into());

        // assert
        assert!(result.is_ok_and(|f| f == "[[path2/nöte1.md]]".into()));
    }
}
