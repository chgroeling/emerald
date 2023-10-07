use crate::{resources::meta_data_loader::MetaDataLoader, types::meta_data::FileType};
use std::rc::Rc;

use crate::types::ResourceId;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::resource_ids_iter_src::ResourceIdsIterSrc;

pub struct ResourceIdIndex<U>
where
    U: MetaDataLoader,
{
    all_resource_ids_list: Vec<ResourceId>,
    md_resource_ids_list: Vec<ResourceId>,
    meta_data_loader: Rc<U>,
}

impl<U> ResourceIdIndex<U>
where
    U: MetaDataLoader,
{
    pub fn new(meta_data_loader: Rc<U>) -> Self {
        let all_resource_ids_list = Vec::<ResourceId>::new();
        let md_resource_ids_list = Vec::<ResourceId>::new();

        Self {
            all_resource_ids_list,
            md_resource_ids_list,
            meta_data_loader,
        }
    }

    pub fn update(&mut self, ep_iter: &impl ResourceIdsIterSrc) {
        let mut all_resource_ids_list = Vec::<ResourceId>::new();
        let mut md_resource_ids_list = Vec::<ResourceId>::new();

        for resource_id in ep_iter.iter() {
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
pub struct AllResourceIds<U>(Rc<ResourceIdIndex<U>>)
where
    U: MetaDataLoader;

impl<U> AllResourceIds<U>
where
    U: MetaDataLoader,
{
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex<U>) -> Self {
        Self(Rc::new(value))
    }
    pub fn new_from_rc(value: &Rc<ResourceIdIndex<U>>) -> Self {
        Self(value.clone())
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
pub struct MdResourceIds<U>(Rc<ResourceIdIndex<U>>)
where
    U: MetaDataLoader;

impl<U> MdResourceIds<U>
where
    U: MetaDataLoader,
{
    #[allow(dead_code)]
    pub fn new(value: ResourceIdIndex<U>) -> Self {
        Self(Rc::new(value))
    }
    pub fn new_from_rc(value: &Rc<ResourceIdIndex<U>>) -> Self {
        Self(value.clone())
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
    /*
    use super::{EndPoint, ResourceId, ResourceIdIndex};
    use crate::indexes::resource_id_index::{AllResourceIds, MdResourceIds, ResourceIdsIterSrc};
    use crate::resources::endpoints_iter_src::MockEndpointsIterSrc;
    use crate::resources::meta_data_loader::MockMetaDataLoader;
    use crate::resources::resource_id_resolver::MockResourceIdResolver;
    use crate::types::meta_data::{FileType, MetaData};
    use crate::EmeraldError;
    use std::path::PathBuf;
    use std::rc::Rc;
    use EndPoint::*;

    fn create_dut(
        test_ep_list: Vec<EndPoint>,
        file_type: Vec<FileType>,
    ) -> ResourceIdIndex<MockResourceIdResolver, MockMetaDataLoader> {
        let mut mock_it_src = MockEndpointsIterSrc::new();
        let mut mock_res_id_res = MockResourceIdResolver::new();

        mock_it_src
            .expect_iter()
            .return_const(test_ep_list.clone().into_iter());

        // iterate test data to set expectations for resolver
        for test_ep in test_ep_list {
            let test_path: PathBuf;
            match &test_ep {
                FileUnknown(ex) => test_path = ex.clone(),
                FileMarkdown(ex) => test_path = ex.clone(),
            }
            let test_path_str = test_path.to_str().unwrap();
            let test_str = format!("[[{test_path_str}]]");
            mock_res_id_res
                .expect_resolve()
                .withf(move |f| f == &test_ep)
                .returning(move |_f| Ok(ResourceId(test_str.clone())));
        }

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
        let mut ridx = ResourceIdIndex::new(
            Rc::new(mock_res_id_res),
            Rc::new(mock_meta_data_loader_load),
        );

        ridx.update(&mock_it_src);

        ridx
    }



    #[test]
    fn test_filter_two_but_one_remains() {
        let dut = MdResourceIds::new(create_dut(
            vec![
                FileUnknown("test_file1.png".into()),
                FileMarkdown("test_file2.md".into()),
            ],
            vec![
                FileType::Unknown("".into()),
                FileType::Markdown("md".into()),
            ],
        ));

        let result: Vec<ResourceId> = dut.iter().collect();
        let expected: Vec<ResourceId> = vec!["[[test_file2.md]]".into()];
        assert_eq!(result, expected);
    }
    */
}
