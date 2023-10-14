use crate::{
    resources::MetaDataLoader,
    types::{FileType, ResourceId},
};

pub fn trafo_to_filetype_and_resource_id<'a>(
    res_id_iter: impl Iterator<Item = ResourceId> + 'a,
    meta_data_loader: &'a impl MetaDataLoader,
) -> impl Iterator<Item = (FileType, ResourceId)> + 'a {
    res_id_iter.map(|f| {
        let res_meta_data = meta_data_loader.load(&f);
        if let Ok(meta_data) = res_meta_data {
            (meta_data.file_type, f.clone())
        } else {
            (FileType::NoFileType(), f.clone())
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        resources::meta_data_loader::MockMetaDataLoader,
        trafos::trafo_to_filetype_and_resource_id,
        types::{FileType, MetaData, ResourceId},
    };

    #[test]
    fn test_trafo_to_filetype_and_resource_id() {
        let all_res_ids = vec![ResourceId("[[rid1]]".into()), ResourceId("[[rid2]]".into())];
        let mut count = 0;

        let mut mock_md_loader = MockMetaDataLoader::new();
        mock_md_loader.expect_load().returning(move |_| {
            count += 1;
            let ft = match count {
                1 => FileType::Unknown("unk".into()),
                2 => FileType::Markdown("md".into()),
                _ => FileType::Unknown("unk".into()),
            };

            Ok(MetaData {
                file_stem: "".into(),
                file_type: ft,
            })
        });

        // Act
        let result = trafo_to_filetype_and_resource_id(all_res_ids.into_iter(), &mock_md_loader);
        let result: Vec<(FileType, ResourceId)> = result.collect();

        // Assert
        let expected: Vec<(FileType, ResourceId)> = vec![
            (FileType::Unknown("unk".into()), "[[rid1]]".into()),
            (FileType::Markdown("md".into()), "[[rid2]]".into()),
        ];
        assert_eq!(result, expected);
    }
}
