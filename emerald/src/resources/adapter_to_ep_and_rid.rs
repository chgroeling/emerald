use crate::error::{EmeraldError::*, Result};
use crate::{types, utils};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::path::Path;

pub fn adapter_ep_to_ep_and_rid<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceObject> + 'a,
    common_path: &'a Path,
) -> Result<impl Iterator<Item = (types::ResourceObject, types::ResourceId)> + 'a> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|ep| {
            let opt_resource_id = utils::convert_endpoint_to_resource_id(ep, common_path);

            if let Ok(resource_id) = opt_resource_id {
                Ok((ep.clone(), resource_id))
            } else {
                error!("Can't convert Endpoint '{:?}' to ResourceId.", &ep);
                Err(ValueError)
            }
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::adapter_ep_to_ep_and_rid;
    use crate::types::ResourceId;
    use crate::types::ResourceObject;
    use std::path::PathBuf;

    #[test]
    fn test_resolve_different_utf8_norm_match() {
        let eps: Vec<_> = vec![ResourceObject::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let res: Vec<_> = adapter_ep_to_ep_and_rid(eps.iter(), &common_path)
            .unwrap()
            .collect();

        assert_eq!(
            res,
            vec![(
                ResourceObject::FileUnknown("testpäth".into()),
                ResourceId("[[testpäth]]".into())
            )]
        );
    }

    #[test]
    fn test_resolve_with_different_utf8_norm_match_2() {
        let eps: Vec<_> = vec![ResourceObject::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let res: Vec<_> = adapter_ep_to_ep_and_rid(eps.iter(), &common_path)
            .unwrap()
            .collect();

        assert_eq!(
            res,
            vec![(
                ResourceObject::FileUnknown("testpäth".into()),
                ResourceId("[[testpäth]]".into())
            )]
        );
    }
}
