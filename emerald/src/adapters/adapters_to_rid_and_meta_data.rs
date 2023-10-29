use crate::error::Result;
use crate::{resources, types};

pub fn adapter_to_rid_and_meta_data<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
    meta_data_loader: &'a impl resources::MetaDataLoader,
) -> Result<impl Iterator<Item = (types::ResourceId, types::MetaData)> + 'a> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|f| -> Result<(types::ResourceId, types::MetaData)> {
            let res_meta_data = meta_data_loader.load(&f)?;
            Ok((f, res_meta_data))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::types::FileType;
    use crate::{adapters::adapter_to_rid_and_meta_data, resources::MockMetaDataLoader, types};

    fn create_test_data(rid: &str, file_type: FileType) -> (types::ResourceId, types::MetaData) {
        (
            rid.into(),
            types::MetaData {
                file_stem: "".into(),
                file_type: file_type,
            },
        )
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
            count += 1;
            let file_type = match count {
                1 => types::FileType::Unknown("unk".into()),
                2 => types::FileType::Markdown("md".into()),
                _ => types::FileType::Unknown("unk".into()),
            };

            Ok(types::MetaData {
                file_stem: "".into(),
                file_type,
            })
        });

        // Act
        let result = adapter_to_rid_and_meta_data(all_res_ids, &mock_md_loader);
        let result: Vec<_> = result.unwrap().collect();

        // Assert
        let expected: Vec<_> = vec![
            create_test_data("[[rid1]]", types::FileType::Unknown("unk".into())),
            create_test_data("[[rid2]]", types::FileType::Markdown("md".into())),
        ];
        assert_eq!(result, expected);
    }
}
