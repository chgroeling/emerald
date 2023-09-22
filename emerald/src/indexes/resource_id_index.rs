use std::path::Path;

use crate::{types::ResourceId, utils::endpoint_translation::convert_endpoint_to_resource_id};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

use super::{
    all_endpoints_iterable::AllEndpointsIterable,
    all_resource_ids_iter_source::AllResourceIdsIterSource,
    md_resource_ids_iter_source::MdResourceIdsIterSource,
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

impl AllResourceIdsIterSource for ResourceIdIndex {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn all_iter(&self) -> Self::Iter {
        self.all_resource_ids_list.clone().into_iter()
    }
}

impl MdResourceIdsIterSource for ResourceIdIndex {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn md_iter(&self) -> Self::Iter {
        self.md_resource_ids_list.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::AllEndpointsIterable;
    use super::EndPoint;
    use super::ResourceId;
    use super::ResourceIdIndex;

    use crate::indexes::resource_id_index::{AllResourceIdsIterSource, MdResourceIdsIterSource};
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
    fn test_all_iter_empty() {
        let common_path = PathBuf::from("");
        let mock = MockEndPointIndex { endpoints: vec![] };

        let dut = ResourceIdIndex::new(&mock, &common_path);

        let result: Vec<ResourceId> = dut.all_iter().collect();
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

        let result: Vec<ResourceId> = dut.all_iter().collect();
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

        let result: Vec<ResourceId> = dut.all_iter().collect();
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

        let result: Vec<ResourceId> = dut.md_iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file2.md]]".into()];

        assert_eq!(result, expected);
    }
}
