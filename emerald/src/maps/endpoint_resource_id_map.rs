use crate::{EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, path::Path};

use crate::{
    indexes::EndpointsIterSrc,
    types::{EndPoint, ResourceId},
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};
use EmeraldError::*;

use super::resource_id_queryable::ResourceIdQuerier;

pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: HashMap<ResourceId, EndPoint>,
}

impl EndpointResourceIdMap {
    pub fn new(ep_iter_rc: &impl EndpointsIterSrc, common_path: &Path) -> Self {
        let mut resource_id_to_endpoint = HashMap::<ResourceId, EndPoint>::new();
        for endpoint in ep_iter_rc.iter() {
            let opt_resource_id = convert_endpoint_to_resource_id(endpoint.clone(), common_path);

            if let Some(resource_id) = opt_resource_id {
                resource_id_to_endpoint.insert(resource_id, endpoint);
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &endpoint);
            }
        }
        Self {
            resource_id_to_endpoint,
        }
    }
}

impl ResourceIdQuerier for EndpointResourceIdMap {
    fn get(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::EndpointResourceIdMap;
    use super::{EmeraldError, EndPoint};
    use crate::indexes::endpoints_iter_src::MockEndpointsIterSrc;
    use crate::maps::resource_id_queryable::ResourceIdQuerier;
    use std::path::PathBuf;
    use EmeraldError::*;

    #[test]
    fn test_get_single() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::File("testpath".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.get(&"[[testpath]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::File(path) if path==PathBuf::from("testpath")));
    }

    #[test]
    fn test_get_single_with_different_utf8_norm_match() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::File("testpäth".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.get(&"[[testpäth]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::File(path) if path==PathBuf::from("testpäth")));
    }

    #[test]
    fn test_get_single_with_different_utf8_norm_fail() {
        let common_path: PathBuf = "".into();
        let test_data: Vec<EndPoint> = vec![EndPoint::File("testpäth".into())];
        let mut mock = MockEndpointsIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = EndpointResourceIdMap::new(&mock, &common_path);
        let ep = dut.get(&"[[testpäth]]".into());

        assert!(matches!(ep, Err(EndPointNotFound)));
    }
}
