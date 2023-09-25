use std::{path::Path, rc::Rc};

use crate::{types::ResourceId, utils::endpoint_translation::convert_endpoint_to_resource_id};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

use super::{endpoints_iterable::EndpointsIterable, resource_ids_iterable::ResourceIdsIterable};

pub struct ResourceIdIndex {
    all_resource_ids_list: Vec<ResourceId>,
    md_resource_ids_list: Vec<ResourceId>,
}

impl ResourceIdIndex {
    pub fn new(ep_iterable: &impl EndpointsIterable, common_path: &Path) -> ResourceIdIndex {
        let mut all_resource_ids_list = Vec::<ResourceId>::new();
        let mut md_resource_ids_list = Vec::<ResourceId>::new();

        for endpoint in ep_iterable.all_iter() {
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

// === Implement trait for all resource ids. =================
pub struct AllResourceIds(Rc<ResourceIdIndex>);
impl AllResourceIds {
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex) -> Self {
        Self(Rc::new(value))
    }
    pub fn new_from_rc(value: &Rc<ResourceIdIndex>) -> Self {
        Self(value.clone())
    }
}
impl ResourceIdsIterable for AllResourceIds {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        self.0.all_resource_ids_list.clone().into_iter()
    }
}

// === Implement trait for md resource ids. =================
pub struct MdResourceIds(Rc<ResourceIdIndex>);

impl MdResourceIds {
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex) -> Self {
        Self(Rc::new(value))
    }
    pub fn new_from_rc(value: &Rc<ResourceIdIndex>) -> Self {
        Self(value.clone())
    }
}

impl ResourceIdsIterable for MdResourceIds {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        self.0.md_resource_ids_list.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{EndPoint, ResourceId, ResourceIdIndex};
    use crate::indexes::endpoints_iterable::MockEndpointsIterable;
    use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdsIterable};
    use std::path::PathBuf;
    use EndPoint::*;

    #[test]
    fn test_md_iter_empty() {
        let common_path = PathBuf::from("");
        let test_data = vec![];
        let mut mock = MockEndpointsIterable::new();
        mock.expect_all_iter().return_const(test_data.into_iter());

        let dut = AllResourceIds::new(ResourceIdIndex::new(&mock, &common_path));
        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one() {
        let common_path = PathBuf::from("");
        let test_data = vec![File("testpath".into())];
        let mut mock = MockEndpointsIterable::new();
        mock.expect_all_iter().return_const(test_data.into_iter());

        let dut = AllResourceIds::new(ResourceIdIndex::new(&mock, &common_path));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[testpath]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let common_path = PathBuf::from("");
        let test_data = vec![File("test_file1".into()), File("test_file2".into())];
        let mut mock = MockEndpointsIterable::new();
        mock.expect_all_iter().return_const(test_data.into_iter());

        let dut = AllResourceIds::new(ResourceIdIndex::new(&mock, &common_path));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file1]]".into(), "[[test_file2]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_two_but_one_remains() {
        let common_path = PathBuf::from("");
        let test_data = vec![
            File("test_file1.png".into()),
            FileMarkdown("test_file2.md".into()),
        ];
        let mut mock = MockEndpointsIterable::new();
        mock.expect_all_iter().return_const(test_data.into_iter());

        let dut = MdResourceIds::new(ResourceIdIndex::new(&mock, &common_path));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file2.md]]".into()];

        assert_eq!(result, expected);
    }
}
