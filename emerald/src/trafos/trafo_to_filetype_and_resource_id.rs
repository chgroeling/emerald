use crate::{
    resources::MetaDataLoader,
    types::{FileType, ResourceId},
};

pub fn trafo_to_filetype_and_res_id<'a>(
    res_id_iter: impl Iterator<Item = &'a ResourceId> + 'a,
    meta_data_loader: &'a impl MetaDataLoader,
) -> impl Iterator<Item = (&'a ResourceId, FileType)> + 'a {
    res_id_iter.map(|f| {
        let res_meta_data = meta_data_loader.load(&f);
        if let Ok(meta_data) = res_meta_data {
            (f, meta_data.file_type)
        } else {
            (f, FileType::NoFileType())
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        resources::meta_data_loader::MockMetaDataLoader,
        trafos::trafo_to_filetype_and_res_id,
        types::{FileType, MetaData, ResourceId},
    };

    #[test]
    fn test_trafo_to_filetype_and_resource_id() {
        let all_res_ids = vec![ResourceId("[[rid1]]".into()), ResourceId("[[rid2]]".into())];
        let mut count = 0;
        let mut mock_md_loader = MockMetaDataLoader::new();
        mock_md_loader.expect_load().returning(move |_| {
            count += 1;
            let file_type = match count {
                1 => FileType::Unknown("unk".into()),
                2 => FileType::Markdown("md".into()),
                _ => FileType::Unknown("unk".into()),
            };

            Ok(MetaData {
                file_stem: "".into(),
                file_type,
            })
        });

        // Act
        let result = trafo_to_filetype_and_res_id(all_res_ids.iter(), &mock_md_loader);
        let result: Vec<(_, _)> = result.collect();

        // Assert
        let rid1: ResourceId = "[[rid1]]".into();
        let rid2: ResourceId = "[[rid2]]".into();
        let expected: Vec<(_, _)> = vec![
            (&rid1, FileType::Unknown("unk".into())),
            (&rid2, FileType::Markdown("md".into())),
        ];
        assert_eq!(result, expected);
    }
}
