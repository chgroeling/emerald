use crate::types::ResourceId;
use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_ids_iter_src::ResourceIdsIterSrc;

#[derive(Clone)]
pub struct ResourceIdIndex {
    all_resource_ids_list: Rc<Vec<ResourceId>>,
    md_resource_ids_list: Rc<Vec<ResourceId>>,
}

impl ResourceIdIndex {
    pub fn new(
        all_resource_ids_list: Vec<ResourceId>,
        md_resource_ids_list: Vec<ResourceId>,
    ) -> Self {
        Self {
            all_resource_ids_list: Rc::new(all_resource_ids_list),
            md_resource_ids_list: Rc::new(md_resource_ids_list),
        }
    }
}

// === Implement trait for all resource ids. =================
#[derive(Clone)]
pub struct AllResourceIds(ResourceIdIndex);

impl AllResourceIds {
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex) -> Self {
        Self(value)
    }
}
impl ResourceIdsIterSrc for AllResourceIds {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        (*self.0.all_resource_ids_list).clone().into_iter()
    }
}

// === Implement trait for md resource ids. =================
#[derive(Clone)]
pub struct MdResourceIds(ResourceIdIndex);

impl MdResourceIds {
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex) -> Self {
        Self(value)
    }
}

impl ResourceIdsIterSrc for MdResourceIds {
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        (*self.0.md_resource_ids_list).clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{MdResourceIds, ResourceIdsIterSrc};

    #[test]
    fn test_filter_two_but_one_remains() {
        let all_res_ids = vec![ResourceId("[[rid1]]".into()), ResourceId("[[rid2]]".into())];
        let md_res_ids = vec![ResourceId("[[rid2]]".into())];
        let dut = MdResourceIds(ResourceIdIndex::new(all_res_ids, md_res_ids));

        // Act
        let result: Vec<ResourceId> = dut.iter().collect();

        // Assert
        let expected: Vec<ResourceId> = vec!["[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
