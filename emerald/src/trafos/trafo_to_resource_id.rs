use crate::{
    resources::resource_id_resolver::ResourceIdResolver,
    types::{EndPoint, FileType, ResourceId},
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn filter_markdown_types<'a>(
    it_src: impl IntoIterator<Item = (&'a ResourceId, FileType)> + 'a,
) -> impl Iterator<Item = &'a ResourceId> + 'a {
    it_src
        .into_iter()
        .filter(|pred| matches!(pred.1, FileType::Markdown(_)))
        .map(|f| f.0)
}

pub fn trafo_ep_to_rid<'a>(
    it_src: impl IntoIterator<Item = &'a EndPoint> + 'a,
    resource_id_resolver: &'a impl ResourceIdResolver,
) -> impl Iterator<Item = ResourceId> + 'a {
    it_src.into_iter().filter_map(|ep| {
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
        let rid1: ResourceId = "[[rid1]]".into();
        let rid2: ResourceId = "[[rid2]]".into();

        let all_res_ids = vec![
            (&rid1, FileType::Unknown("md".into())),
            (&rid2, FileType::Markdown("md".into())),
        ];

        // Act
        let result: Vec<_> = filter_markdown_types(all_res_ids.into_iter()).collect();

        // Assert
        let rid2_exp: ResourceId = "[[rid2]]".into();
        let expected: Vec<&ResourceId> = vec![&rid2_exp];
        assert_eq!(result, expected);
    }
}
