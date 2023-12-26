use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn filter_rid_and_meta_data<'a>(
    it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)> + 'a,
) -> impl Iterator<Item = (types::ResourceId, types::FilesystemMetadata)> + 'a {
    it_src
        .into_iter()
        .filter(|pred| matches!(pred.1.resource_type, types::ResourceType::Markdown()))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::types;
    use crate::adapters::filter_rid_and_meta_data;

    pub fn create_meta_data(resource_type: types::ResourceType) -> types::FilesystemMetadata {
        types::FilesystemMetadataBuilder::new()
            .set_resource_type(resource_type)
            .build()
    }

    fn create_rid_meta_data(
        rid: &str,
        ft: types::ResourceType,
    ) -> (types::ResourceId, types::FilesystemMetadata) {
        (rid.into(), create_meta_data(ft))
    }

    #[test]
    fn test_filter_two_and_one_remains() {
        use types::ResourceType::*;

        let all_res_ids = vec![
            create_rid_meta_data("[[rid1]]", Unknown()),
            create_rid_meta_data("[[rid2]]", Markdown()),
        ];

        let result: Vec<_> = filter_rid_and_meta_data(all_res_ids.iter()).collect();

        assert_eq!(result, vec![create_rid_meta_data("[[rid2]]", Markdown())]);
    }
}
