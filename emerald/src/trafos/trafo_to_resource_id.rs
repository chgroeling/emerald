use crate::types::{meta_data::FileType, ResourceId};

pub fn filter_markdown_types<'a>(
    iter: impl Iterator<Item = (FileType, ResourceId)> + 'a,
) -> impl Iterator<Item = ResourceId> + 'a {
    iter.filter(|pred| matches!(pred.0, FileType::Markdown(_)))
        .map(|f| f.1)
}

#[cfg(test)]
mod tests {
    use super::ResourceId;
    use crate::{trafos::filter_markdown_types, types::meta_data::FileType};

    #[test]
    fn test_filter_markdown_types_two_but_one_remains() {
        let all_res_ids = vec![
            (
                FileType::Unknown("md".into()),
                ResourceId("[[rid1]]".into()),
            ),
            (
                FileType::Markdown("md".into()),
                ResourceId("[[rid2]]".into()),
            ),
        ];

        // Act
        let result = filter_markdown_types(all_res_ids.into_iter());
        let result: Vec<ResourceId> = result.collect();

        // Assert
        let expected: Vec<ResourceId> = vec!["[[rid2]]".into()];
        assert_eq!(result, expected);
    }
}
