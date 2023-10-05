use std::{path::Path, rc::Rc};

use crate::{
    resources::endpoints_iter_src::EndpointsIterSrc, types::ResourceId,
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

use super::resource_ids_iter_src::ResourceIdsIterSrc;

pub struct ResourceIdIndex {
    all_resource_ids_list: Vec<ResourceId>,
    md_resource_ids_list: Vec<ResourceId>,
}

impl ResourceIdIndex {
    pub fn new(ep_iter_rc: &impl EndpointsIterSrc, common_path: &Path) -> ResourceIdIndex {
        let mut all_resource_ids_list = Vec::<ResourceId>::new();
        let mut md_resource_ids_list = Vec::<ResourceId>::new();

        for endpoint in ep_iter_rc.iter() {
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
impl ResourceIdsIterSrc for AllResourceIds {
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

impl ResourceIdsIterSrc for MdResourceIds {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        self.0.md_resource_ids_list.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{EndPoint, ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdsIterSrc};
    use crate::resources::endpoints_iter_src::MockEndpointsIterSrc;
    use std::path::PathBuf;
    use EndPoint::*;

    fn setup_dut(test_ep: Vec<EndPoint>) -> ResourceIdIndex {
        let common_path = PathBuf::from("");
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_ep.clone().into_iter());
        let dut = ResourceIdIndex::new(&mock, &common_path);
        dut
    }

    #[test]
    fn test_md_iter_empty() {
        let dut = AllResourceIds::new(setup_dut(vec![]));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one() {
        let dut = AllResourceIds::new(setup_dut(vec![FileUnknown("testpath".into())]));
        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[testpath]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let dut = AllResourceIds::new(setup_dut(vec![
            FileUnknown("test_file1".into()),
            FileUnknown("test_file2".into()),
        ]));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file1]]".into(), "[[test_file2]]".into()];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_two_but_one_remains() {
        let dut = MdResourceIds::new(setup_dut(vec![
            FileUnknown("test_file1.png".into()),
            FileMarkdown("test_file2.md".into()),
        ]));
        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file2.md]]".into()];

        assert_eq!(result, expected);
    }
}
