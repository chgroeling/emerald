use crate::types::ResourceId;
use crate::{resources::meta_data_loader::MetaDataLoader, types::meta_data::FileType};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_ids_iter_src::ResourceIdsIterSrc;

#[derive(Clone)]
pub struct ResourceIdIndex<U>
where
    U: MetaDataLoader,
{
    all_resource_ids_list: Vec<ResourceId>,
    md_resource_ids_list: Vec<ResourceId>,
    meta_data_loader: U,
}

impl<U> ResourceIdIndex<U>
where
    U: MetaDataLoader,
{
    pub fn new(meta_data_loader: U) -> Self {
        let all_resource_ids_list = Vec::<ResourceId>::new();
        let md_resource_ids_list = Vec::<ResourceId>::new();

        Self {
            all_resource_ids_list,
            md_resource_ids_list,
            meta_data_loader,
        }
    }

    pub fn update(&mut self, resource_iter_src: &impl ResourceIdsIterSrc) {
        let mut all_resource_ids_list = Vec::<ResourceId>::new();
        let mut md_resource_ids_list = Vec::<ResourceId>::new();

        for resource_id in resource_iter_src.iter() {
            all_resource_ids_list.push(resource_id.clone());

            let res_meta_data = self.meta_data_loader.load(&resource_id);
            let Ok(meta_data) = res_meta_data else {
                /*warn!(
                    "No meta_data available for '{:?}'. Error: {:?}",
                    &resource_id, res_meta_data
                );*/
                continue;
            };

            if let FileType::Markdown(_) = meta_data.file_type {
                md_resource_ids_list.push(resource_id);
            }
        }

        self.all_resource_ids_list = all_resource_ids_list;
        self.md_resource_ids_list = md_resource_ids_list;
    }
}

// === Implement trait for all resource ids. =================
pub struct AllResourceIds<U>(ResourceIdIndex<U>)
where
    U: MetaDataLoader;

impl<U> AllResourceIds<U>
where
    U: MetaDataLoader,
{
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex<U>) -> Self {
        Self(value)
    }
}
impl<U> ResourceIdsIterSrc for AllResourceIds<U>
where
    U: MetaDataLoader,
{
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        self.0.all_resource_ids_list.clone().into_iter()
    }
}

// === Implement trait for md resource ids. =================
pub struct MdResourceIds<U>(ResourceIdIndex<U>)
where
    U: MetaDataLoader;

impl<U> MdResourceIds<U>
where
    U: MetaDataLoader,
{
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex<U>) -> Self {
        Self(value)
    }
}

impl<U> ResourceIdsIterSrc for MdResourceIds<U>
where
    U: MetaDataLoader,
{
    type Iter = std::vec::IntoIter<ResourceId>;
    fn iter(&self) -> Self::Iter {
        self.0.md_resource_ids_list.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::{ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{MdResourceIds, ResourceIdsIterSrc};
    use crate::indexes::resource_ids_iter_src::MockResourceIdsIterSrc;
    use crate::resources::meta_data_loader::MockMetaDataLoader;
    use crate::types::meta_data::{FileType, MetaData};
    use crate::EmeraldError;

    fn create_dut(file_type: Vec<FileType>) -> ResourceIdIndex<MockMetaDataLoader> {
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
        ResourceIdIndex::new(mock_meta_data_loader_load)
    }

    #[test]
    fn test_filter_two_but_one_remains() {
        // Arrange
        let mut obj = create_dut(vec![
            FileType::Unknown("".into()),
            FileType::Markdown("md".into()),
        ]);

        let mut mock_it_src = MockResourceIdsIterSrc::new();
        let res_ids = vec![ResourceId("[[rid1]]".into()), ResourceId("[[rid2]]".into())];
        mock_it_src
            .expect_iter()
            .return_const(res_ids.clone().into_iter());

        obj.update(&mock_it_src);
        let dut = MdResourceIds::new(obj);

        // Act
        let result: Vec<ResourceId> = dut.iter().collect();

        // Assert
        let expected: Vec<ResourceId> = vec!["[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
