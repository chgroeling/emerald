use crate::{resources, types};

pub fn adapter_to_rid_and_filetype<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId> + 'a,
    meta_data_loader: &'a impl resources::MetaDataLoader,
) -> impl Iterator<Item = (types::ResourceId, types::FileType)> + 'a {
    it_src.into_iter().map(|f| {
        let res_meta_data = meta_data_loader.load(f);
        if let Ok(meta_data) = res_meta_data {
            (f.clone(), meta_data.file_type)
        } else {
            (f.clone(), types::FileType::NoFileType())
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{adapters::adapter_to_rid_and_filetype, resources::MockMetaDataLoader, types};

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
        let result = adapter_to_rid_and_filetype(&all_res_ids, &mock_md_loader);
        let result: Vec<_> = result.collect();

        // Assert
        let rid1: types::ResourceId = "[[rid1]]".into();
        let rid2: types::ResourceId = "[[rid2]]".into();
        let expected: Vec<_> = vec![
            (rid1, types::FileType::Unknown("unk".into())),
            (rid2, types::FileType::Markdown("md".into())),
        ];
        assert_eq!(result, expected);
    }
}
