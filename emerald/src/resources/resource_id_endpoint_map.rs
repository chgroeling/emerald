use crate::{EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, path::Path, rc::Rc};

use crate::{
    types::{EndPoint, ResourceId},
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};
use EmeraldError::*;

use super::resource_id_resolver::ResourceIdRetriever;

#[derive(Clone)]
pub struct ResourceIdEndPointMap {
    ep_to_resource_id: Rc<HashMap<EndPoint, ResourceId>>,
}

impl ResourceIdEndPointMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = &'a EndPoint>, common_path: &Path) -> Self {
        let mut ep_to_resource_id = HashMap::<EndPoint, ResourceId>::new();
        for ep in it_src.into_iter() {
            let opt_resource_id = convert_endpoint_to_resource_id(ep.clone(), common_path);

            if let Some(resource_id) = opt_resource_id {
                ep_to_resource_id.insert(ep.clone(), resource_id);
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &ep);
            }
        }
        Self {
            ep_to_resource_id: Rc::new(ep_to_resource_id),
        }
    }
}

impl ResourceIdRetriever for ResourceIdEndPointMap {
    fn resolve(&self, ep: &EndPoint) -> Result<ResourceId> {
        self.ep_to_resource_id
            .get(ep)
            .map_or(Err(EndpointHasNoResourceId(ep.clone())), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::resources::resource_id_endpoint_map::ResourceIdEndPointMap;
    use crate::resources::resource_id_resolver::ResourceIdRetriever;
    use crate::types::EndPoint;
    use crate::types::ResourceId;
    use std::path::PathBuf;

    #[test]
    fn test_resolve_different_utf8_norm_match() {
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let dut = ResourceIdEndPointMap::new(test_data.iter(), &common_path);
        let ep = dut
            .resolve(&EndPoint::FileUnknown("testpäth".into()))
            .unwrap();
        assert_eq!(ep, ResourceId("[[testpäth]]".into()));
    }

    #[test]
    fn test_resolve_with_different_utf8_norm_match_2() {
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let common_path: PathBuf = "".into();

        let dut = ResourceIdEndPointMap::new(test_data.iter(), &common_path);
        let ep = dut
            .resolve(&EndPoint::FileUnknown("testpäth".into()))
            .unwrap();
        assert_eq!(ep, ResourceId("[[testpäth]]".into()));
    }
}
