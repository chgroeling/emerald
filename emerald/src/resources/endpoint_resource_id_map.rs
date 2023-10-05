use crate::{
    resources::{
        endpoints_iter_src::EndpointsIterSrc,
        resource_id_resolver::{self, ResourceIdResolver},
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

use super::endpoint_resolver::EndPointResolver;

pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: HashMap<ResourceId, EndPoint>,
}

impl EndpointResourceIdMap {
    pub fn new(
        ep_iter_src: &impl EndpointsIterSrc,
        resource_id_resolver: &impl ResourceIdResolver,
    ) -> Self {
        let mut resource_id_to_endpoint = HashMap::<ResourceId, EndPoint>::new();
        for ep in ep_iter_src.iter() {
            let opt_resource_id = resource_id_resolver.resolve(&ep);

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

impl EndPointResolver for EndpointResourceIdMap {
    fn resolve(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::EndpointResourceIdMap;
    use super::{EmeraldError, EndPoint};
    use crate::resources::endpoint_resolver::EndPointResolver;
    use crate::resources::endpoints_iter_src::MockEndpointsIterSrc;
    use crate::resources::resource_id_resolver::MockResourceIdResolver;
    use crate::types::ResourceId;
    use std::path::PathBuf;
    use EmeraldError::*;

    fn create_dut(test_ep_list: Vec<EndPoint>) -> EndpointResourceIdMap {
        let mut mock_it_src = MockEndpointsIterSrc::new();
        mock_it_src
            .expect_iter()
            .return_const(test_ep_list.into_iter());

        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_resolve()
            .withf(|f| matches!(f, EndPoint::FileUnknown(path) if path==&PathBuf::from("testpath")))
            .returning(|_f| Ok(ResourceId("[[testpath]]".to_string())));

        let dut = EndpointResourceIdMap::new(&mock_it_src, &mock_res_id_res);
        dut
    }
    #[test]
    fn test_resolve_single_entry() {
        let dut = create_dut(vec![EndPoint::FileUnknown("testpath".into())]);
        let ep = dut.resolve(&"[[testpath]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::FileUnknown(path) if path==PathBuf::from("testpath")));
    }

    #[test]
    fn test_new_correct_iteration() {
        let test_ep_list: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpath".into())];
        let mut mock_it_src = MockEndpointsIterSrc::new();
        mock_it_src
            .expect_iter()
            .times(1)
            .return_const(test_ep_list.into_iter());

        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_resolve()
            .returning(|_f| Ok(ResourceId("[[doesnt matter]]".to_string())));

        let _dut = EndpointResourceIdMap::new(&mock_it_src, &mock_res_id_res);
    }

    #[test]
    fn test_new_resolve() {
        let test_ep_list: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpath".into())];
        let mut mock_it_src = MockEndpointsIterSrc::new();
        mock_it_src
            .expect_iter()
            .return_const(test_ep_list.into_iter());

        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_resolve()
            .withf(|f| matches!(f, EndPoint::FileUnknown(path) if path==&PathBuf::from("testpath")))
            .returning(|_f| Ok(ResourceId("[[doesnt matter]]".to_string())));

        let _dut = EndpointResourceIdMap::new(&mock_it_src, &mock_res_id_res);
    }
}
