use std::rc::Rc;

use crate::types::ResourceId;
use crate::{resources::meta_data_loader::MetaDataLoader, types::meta_data::FileType};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_ids_iter_src::ResourceIdsIterSrc;

pub fn transform_to_filetype_and_resource_id<'a>(
    iter: impl Iterator<Item = ResourceId> + 'a,
    meta_data_loader: &'a impl MetaDataLoader,
) -> impl Iterator<Item = (FileType, ResourceId)> + 'a {
    iter.map(|f| {
        let res_meta_data = meta_data_loader.load(&f);
        if let Ok(meta_data) = res_meta_data {
            (meta_data.file_type, f.clone())
        } else {
            (FileType::NoFileType(), f.clone())
        }
    })
}

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
    use super::{transform_to_filetype_and_resource_id, ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{MdResourceIds, ResourceIdsIterSrc};
    use crate::resources::meta_data_loader::MockMetaDataLoader;
    use crate::types::meta_data::{FileType, MetaData};
    use crate::EmeraldError;

    fn create_dut(file_type: Vec<FileType>, resource_ids: Vec<ResourceId>) -> ResourceIdIndex {
        let mut mock_meta_data_loader_load = MockMetaDataLoader::new();
        let mut call_count_meta_data = 0;

        mock_meta_data_loader_load
            .expect_load()
            .returning(move |_| {
                let meta_data = file_type
                    .get(call_count_meta_data)
                    .ok_or_else(|| EmeraldError::Unknown)
                    .map(|ft| MetaData {
                        file_stem: "".into(),
                        file_type: ft.clone(),
                    });
                call_count_meta_data += 1;
                meta_data
            });

        let iter = transform_to_filetype_and_resource_id(
            resource_ids.clone().into_iter(),
            &mock_meta_data_loader_load,
        );
        ResourceIdIndex::new(iter)
    }

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
        let obj = create_dut(file_types, res_ids);
        let dut = MdResourceIds::new(obj);

        // Act
        let result: Vec<ResourceId> = dut.iter().collect();

        // Assert
        let expected: Vec<ResourceId> = vec!["[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
