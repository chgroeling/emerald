use crate::{
    resources::resource_id_resolver::ResourceIdResolver,
    types::{EndPoint, FileType, ResourceId},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn filter_markdown_types<'a>(
    iter: impl Iterator<Item = (FileType, ResourceId)> + 'a,
) -> impl Iterator<Item = ResourceId> + 'a {
    iter.filter(|pred| matches!(pred.0, FileType::Markdown(_)))
        .map(|f| f.1)
}

pub fn trafo_ep_to_rid<'a>(
    ep_iter: impl Iterator<Item = &'a EndPoint> + 'a,
    resource_id_resolver: &'a impl ResourceIdResolver,
) -> impl Iterator<Item = ResourceId> + 'a {
    ep_iter.filter_map(|ep| {
        let opt_resource_id = resource_id_resolver.resolve(ep);
        if let Ok(resource_id) = opt_resource_id {
            return Some(resource_id);
        }

        warn!(
            "Obtaining resource id for endpoint {:?} yielded {:?} ",
            ep, opt_resource_id
        );
        None
    })
}

#[cfg(test)]
mod tests {
    use super::ResourceId;
    use crate::{trafos::filter_markdown_types, types::FileType};

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
