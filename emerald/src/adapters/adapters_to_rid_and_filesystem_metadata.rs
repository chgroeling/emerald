use crate::error::Result;
use crate::{resources, types};

pub fn adapter_to_rid_and_filesystem_metadata<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
    meta_data_loader: &'a impl resources::FilesystemMetadataLoader,
) -> Result<impl Iterator<Item = (types::ResourceId, types::FilesystemMetadata)> + 'a> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(
            |f| -> Result<(types::ResourceId, types::FilesystemMetadata)> {
                let res_meta_data = meta_data_loader.load(&f)?;
                Ok((f.clone(), res_meta_data))
            },
        )
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::types;
    use crate::{
        adapters::adapter_to_rid_and_filesystem_metadata, resources::MockFilesystemMetadataLoader,
    };

    pub fn create_meta_data(resource_type: types::ResourceType) -> types::FilesystemMetadata {
        types::FilesystemMetadataBuilder::new()
            .set_resource_type(resource_type)
            .build()
    }

    fn create_rid_and_meta_data(
        rid: &str,
        resource_type: types::ResourceType,
    ) -> (types::ResourceId, types::FilesystemMetadata) {
        (rid.into(), create_meta_data(resource_type))
    }

    #[test]
    fn test_trafo_to_filetype_and_resource_id() {
        let all_res_ids = vec![
            types::ResourceId("[[rid1]]".into()),
            types::ResourceId("[[rid2]]".into()),
        ];
        let mut count = 0;
        let mut mock_md_loader = MockFilesystemMetadataLoader::new();
        mock_md_loader.expect_load().returning(move |_| {
            use types::ResourceType;
            count += 1;
            Ok(match count {
                1 => create_meta_data(ResourceType::Unknown()),
                2 => create_meta_data(ResourceType::Markdown()),
                _ => create_meta_data(ResourceType::Unknown()),
            })
        });

        // Act
        let result = adapter_to_rid_and_filesystem_metadata(all_res_ids, &mock_md_loader);
        let result: Vec<_> = result.unwrap().collect();

        // Assert
        let expected: Vec<_> = vec![
            create_rid_and_meta_data("[[rid1]]", types::ResourceType::Unknown()),
            create_rid_and_meta_data("[[rid2]]", types::ResourceType::Markdown()),
        ];
        assert_eq!(result, expected);
    }
}
