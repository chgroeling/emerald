use std::{path::Path, rc::Rc};

use crate::{types::ResourceId, utils::endpoint_translation::convert_endpoint_to_resource_id};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

use super::{
    all_endpoints_iterable::AllEndpointsIterable, resource_ids_iterable::ResourceIdsIterable,
};

pub type ResourceIdList = Vec<ResourceId>;

pub struct ResourceIdIndex {
    all_resource_ids_list: ResourceIdList,
    md_resource_ids_list: ResourceIdList,
}

impl ResourceIdIndex {
    pub fn new(
        endpoints_iterable: &impl AllEndpointsIterable,
        common_path: &Path,
    ) -> ResourceIdIndex {
        let mut all_resource_ids_list = ResourceIdList::new();
        let mut md_resource_ids_list = ResourceIdList::new();

        for endpoint in endpoints_iterable.all_iter() {
            let opt_resource_id = convert_endpoint_to_resource_id(endpoint.clone(), common_path);

            if let Some(resource_id) = opt_resource_id {
                all_resource_ids_list.push(resource_id.clone());

                if let EndPoint::FileMarkdown(_) = endpoint {
                    md_resource_ids_list.push(resource_id);
                }
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &endpoint);
            }
        }
        Self {
            all_resource_ids_list,
            md_resource_ids_list,
        }
    }
}

// --------------------------------------------
// Implement trait for all resource ids.
// --------------------------------------------
pub struct AllResourceIds<'a>(&'a ResourceIdIndex);

impl<'a> From<&'a ResourceIdIndex> for AllResourceIds<'a> {
    fn from(value: &'a ResourceIdIndex) -> Self {
        AllResourceIds(value)
    }
}
impl<'a> ResourceIdsIterable for AllResourceIds<'a> {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn md_iter(&self) -> Self::Iter {
        self.0.all_resource_ids_list.clone().into_iter()
    }
}

// --------------------------------------------
// Implement trait for all resource ids.
// --------------------------------------------
pub struct MdResourceIds<'a>(&'a ResourceIdIndex);

impl<'a> From<&'a ResourceIdIndex> for MdResourceIds<'a> {
    fn from(value: &'a ResourceIdIndex) -> Self {
        MdResourceIds(value)
    }
}

impl<'a> ResourceIdsIterable for MdResourceIds<'a> {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn md_iter(&self) -> Self::Iter {
        self.0.md_resource_ids_list.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::AllEndpointsIterable;
    use super::EndPoint;
    use super::ResourceId;
    use super::ResourceIdIndex;

    use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdsIterable};
    use std::path::PathBuf;

    use EndPoint::*;

    struct MockEndPointIndex {
        endpoints: Vec<EndPoint>,
    }

    impl AllEndpointsIterable for MockEndPointIndex {
        type Iter = std::vec::IntoIter<EndPoint>;
        fn all_iter(&self) -> Self::Iter {
            self.endpoints.clone().into_iter()
        }
    }

    #[test]
    fn test_md_iter_empty() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex { endpoints: vec![] };

        let dut = ResourceIdIndex::new(&mock, &common_path);
        let result: Vec<ResourceId> = AllResourceIds::from(&dut).md_iter().collect();
        let expected: Vec<ResourceId> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one() {
        let common_path = PathBuf::from("");

        let mock = MockEndPointIndex {
            endpoints: vec![File("testpath".into())],
        };

        let dut = ResourceIdIndex::new(&mock, &common_path);

        let result: Vec<ResourceId> = AllResourceIds::from(&dut).md_iter().collect();
        let expected: Vec<ResourceId> = vec!["[[testpath]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let common_path = PathBuf::from("");

        let mock = MockEndPointIndex {
            endpoints: vec![File("test_file1".into()), File("test_file2".into())],
        };

        let dut = ResourceIdIndex::new(&mock, &common_path);

        let result: Vec<ResourceId> = AllResourceIds::from(&dut).md_iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file1]]".into(), "[[test_file2]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_two_but_one_remains() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex {
            endpoints: vec![
                File("test_file1.png".into()),
                FileMarkdown("test_file2.md".into()),
            ],
        };

        let dut = ResourceIdIndex::new(&mock, &common_path);

        let result: Vec<ResourceId> = MdResourceIds::from(&dut).md_iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file2.md]]".into()];

        assert_eq!(result, expected);
    }
}
