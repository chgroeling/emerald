use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_rid_and_file_type_to_rid<'a>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, types::FileType)> + 'a,
) -> impl Iterator<Item = &'a types::ResourceId> + 'a {
    it_src
        .into_iter()
        .filter(|pred| matches!(pred.1, types::FileType::Markdown(_)))
        .map(|f| f.0)
}

#[cfg(test)]
mod tests {
    use super::types;
    use crate::{adapters::adapter_rid_and_file_type_to_rid, types::FileType};

    #[test]
    fn test_filter_markdown_types_two_but_one_remains() {
        let rid1: types::ResourceId = "[[rid1]]".into();
        let rid2: types::ResourceId = "[[rid2]]".into();

        let all_res_ids = vec![
            (&rid1, FileType::Unknown("md".into())),
            (&rid2, FileType::Markdown("md".into())),
        ];

        // Act
        let result: Vec<_> = adapter_rid_and_file_type_to_rid(all_res_ids.into_iter()).collect();

        // Assert
        let rid2_exp: types::ResourceId = "[[rid2]]".into();
        let expected: Vec<&types::ResourceId> = vec![&rid2_exp];
        assert_eq!(result, expected);
    }
}
