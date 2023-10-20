use crate::{EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, path::Path, rc::Rc};

use crate::{
    types::{EndPoint, ResourceId},
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};
use EmeraldError::*;

pub fn adapter_ep_to_ep_and_resid<'a>(
    it_src: impl IntoIterator<Item = &'a EndPoint> + 'a,
    common_path: &'a Path,
) -> Result<impl Iterator<Item = (&'a EndPoint, ResourceId)> + 'a> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|ep| {
            let opt_resource_id = convert_endpoint_to_resource_id(ep, common_path);

            if let Ok(resource_id) = opt_resource_id {
                Ok((ep, resource_id))
            } else {
                error!("Can't convert Endpoint '{:?}' to ResourceId.", &ep);
                Err(EmeraldError::ValueError)
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
    use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
    use crate::resources::resource_id_retriever::ResourceIdRetriever;
    use crate::types::EndPoint;
    use crate::types::ResourceId;
    use std::path::PathBuf;
    /*
    #[test]
    fn test_resolve_different_utf8_norm_match() {
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let dut = ResourceIdEndPointMap::new(test_data.iter(), &common_path);
        let ep = dut
            .retrieve(&EndPoint::FileUnknown("testpäth".into()))
            .unwrap();
        assert_eq!(ep, ResourceId("[[testpäth]]".into()));
    }

    #[test]
    fn test_resolve_with_different_utf8_norm_match_2() {
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let dut = ResourceIdEndPointMap::new(test_data.iter(), &common_path);
        let ep = dut
            .retrieve(&EndPoint::FileUnknown("testpäth".into()))
            .unwrap();
        assert_eq!(ep, ResourceId("[[testpäth]]".into()));
    }
    */
}
