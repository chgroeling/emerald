use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn filter_rid_and_meta_data<'a>(
    it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::MetaData)> + 'a,
) -> impl Iterator<Item = (types::ResourceId, types::MetaData)> + 'a {
    it_src
        .into_iter()
        .filter(|pred| matches!(pred.1.file_type, types::FileType::Markdown(_)))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::types;
    use crate::adapters::filter_rid_and_meta_data;

    pub fn create_meta_data(file_type: types::FileType) -> types::MetaData {
        types::MetaData {
            file_stem: "".into(),
            file_type,
        }
    }

    fn create_rid_and_meta_data(
        rid: &str,
        file_type: types::FileType,
    ) -> (types::ResourceId, types::MetaData) {
        (rid.into(), create_meta_data(file_type))
    }

    #[test]
    fn test_filter_markdown_types_two_but_one_remains() {
        let all_res_ids = vec![
            create_rid_and_meta_data("[[rid1]]", types::FileType::Unknown("unk".into())),
            create_rid_and_meta_data("[[rid2]]", types::FileType::Markdown("md".into())),
        ];

        // Act
        let result: Vec<_> = filter_rid_and_meta_data(all_res_ids.iter()).collect();

        // Assert
        let expected: Vec<_> = vec![create_rid_and_meta_data(
            "[[rid2]]",
            types::FileType::Markdown("md".into()),
        )];

        assert_eq!(result, expected);
    }
}
