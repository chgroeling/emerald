use std::rc::Rc;

use crate::types::meta_data::FileType;
use crate::types::ResourceId;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_ids_iter_src::ResourceIdsIterSrc;

#[derive(Clone)]
pub struct ResourceIdIndex {
    all_resource_ids_list: Rc<Vec<ResourceId>>,
    md_resource_ids_list: Rc<Vec<ResourceId>>,
}

impl ResourceIdIndex {
    pub fn new(iter: impl Iterator<Item = (FileType, ResourceId)>) -> Self {
        let mut all_resource_ids_list = Vec::<ResourceId>::new();
        let mut md_resource_ids_list = Vec::<ResourceId>::new();

        for (file_type, resource_id) in iter {
            all_resource_ids_list.push(resource_id.clone());
            if let FileType::Markdown(_) = file_type {
                md_resource_ids_list.push(resource_id);
            }
        }

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
    use std::iter::zip;

    use super::{ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{MdResourceIds, ResourceIdsIterSrc};
    use crate::types::meta_data::FileType;

    #[test]
    fn test_filter_two_but_one_remains() {
        // Arrange
        let file_types = vec![
            FileType::Unknown("".into()),
            FileType::Markdown("md".into()),
        ];
        let res_ids = vec![
            ResourceId("[[rid1]]".into()), //
            ResourceId("[[rid2]]".into()),
        ];
        let iter = zip(file_types.into_iter(), res_ids.into_iter());
        let dut = MdResourceIds(ResourceIdIndex::new(iter));

        // Act
        let result: Vec<ResourceId> = dut.iter().collect();

        // Assert
        let expected: Vec<ResourceId> = vec!["[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
