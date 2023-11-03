use crate::error::Result;
use crate::{resources, types};

pub fn adapter_to_rid_and_meta_data<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId> + 'a,
    meta_data_loader: &'a impl resources::MetaDataLoader,
) -> Result<impl Iterator<Item = (types::ResourceId, types::MetaData)> + 'a> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|f| -> Result<(types::ResourceId, types::MetaData)> {
            let res_meta_data = meta_data_loader.load(f)?;
            Ok((f.clone(), res_meta_data))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::types;
    use crate::{adapters::adapter_to_rid_and_meta_data, resources::MockMetaDataLoader};

    pub fn create_meta_data(resource_type: types::ResourceType) -> types::MetaData {
        types::MetaDataBuilder::new()
            .set_name("".into())
            .set_resource_type(resource_type)
            .build()
    }

    fn create_rid_and_meta_data(
        rid: &str,
        resource_type: types::ResourceType,
    ) -> (types::ResourceId, types::MetaData) {
        (rid.into(), create_meta_data(resource_type))
    }

    #[test]
    fn test_trafo_to_filetype_and_resource_id() {
        let all_res_ids = vec![
            types::ResourceId("[[rid1]]".into()),
            types::ResourceId("[[rid2]]".into()),
        ];
        let mut count = 0;
        let mut mock_md_loader = MockMetaDataLoader::new();
        mock_md_loader.expect_load().returning(move |_| {
            use types::ResourceType;
            count += 1;
            Ok(match count {
                1 => create_meta_data(ResourceType::Unknown("unk".into())),
                2 => create_meta_data(ResourceType::Markdown("md".into())),
                _ => create_meta_data(ResourceType::Unknown("unk".into())),
            })
        });

        // Act
        let result = adapter_to_rid_and_meta_data(&all_res_ids, &mock_md_loader);
        let result: Vec<_> = result.unwrap().collect();

        // Assert
        let expected: Vec<_> = vec![
            create_rid_and_meta_data("[[rid1]]", types::ResourceType::Unknown("unk".into())),
            create_rid_and_meta_data("[[rid2]]", types::ResourceType::Markdown("md".into())),
        ];
        assert_eq!(result, expected);
    }
}
