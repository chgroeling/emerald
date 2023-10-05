use crate::{
    resources::{
        endpoints_iter_src::EndpointsIterSrc,
        resource_id_getter::{self, ResourceIdGetter},
    },
    EmeraldError, Result,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, path::Path};

use crate::{
    types::{EndPoint, ResourceId},
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};
use EmeraldError::*;

use super::endpoint_retriever::EndPointRetriever;

pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: HashMap<ResourceId, EndPoint>,
}

impl EndpointResourceIdMap {
    pub fn new(ep_iter_src: &impl EndpointsIterSrc, res_id_getter: &impl ResourceIdGetter) -> Self {
        let mut resource_id_to_endpoint = HashMap::<ResourceId, EndPoint>::new();
        for ep in ep_iter_src.iter() {
            let opt_resource_id = res_id_getter.get(&ep);

            if let Ok(resource_id) = opt_resource_id {
                resource_id_to_endpoint.insert(resource_id, ep);
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &ep);
            }
        }
        Self {
            resource_id_to_endpoint,
        }
    }
}

impl EndPointRetriever for EndpointResourceIdMap {
    fn retrieve(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::EndpointResourceIdMap;
    use super::{EmeraldError, EndPoint};
    use crate::resources::endpoint_retriever::EndPointRetriever;
    use crate::resources::endpoints_iter_src::MockEndpointsIterSrc;
    use std::path::PathBuf;
    use EmeraldError::*;
    /* TODO
    #[test]
    fn test_get_single() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpath".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.retrieve(&"[[testpath]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::FileUnknown(path) if path==PathBuf::from("testpath")));
    }

    #[test]
    fn test_get_single_with_different_utf8_norm_match() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.retrieve(&"[[testpäth]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::FileUnknown(path) if path==PathBuf::from("testpäth")));
    }

    #[test]
    fn test_get_single_with_different_utf8_norm_fail() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpäth".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.retrieve(&"[[testpäth]]".into());

        assert!(matches!(ep, Err(EndPointNotFound)));
    }
    */
}
