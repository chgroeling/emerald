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
        types::MetaDataBuilder::new()
            .set_file_stem("".into())
            .set_file_type(file_type)
            .build()
    }

    fn create_rid_meta_data(
        rid: &str,
        ft: types::FileType,
    ) -> (types::ResourceId, types::MetaData) {
        (rid.into(), create_meta_data(ft))
    }

    #[test]
    fn test_filter_two_and_one_remains() {
        use types::FileType::*;

        let all_res_ids = vec![
            create_rid_meta_data("[[rid1]]", Unknown("unk".into())),
            create_rid_meta_data("[[rid2]]", Markdown("md".into())),
        ];

        let result: Vec<_> = filter_rid_and_meta_data(all_res_ids.iter()).collect();

        assert_eq!(
            result,
            vec![create_rid_meta_data("[[rid2]]", Markdown("md".into()))]
        );
    }
}
