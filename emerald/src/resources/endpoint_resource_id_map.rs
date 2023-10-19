use crate::{resources::resource_id_retriever::ResourceIdRetriever, EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

use crate::types::{EndPoint, ResourceId};
use EmeraldError::*;

use super::endpoint_retriever::EndpointRetriever;

#[derive(Clone)]
pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: Rc<HashMap<ResourceId, EndPoint>>,
}

impl EndpointResourceIdMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a EndPoint>,
        resource_id_retriever: &impl ResourceIdRetriever,
    ) -> Self {
        let mut resource_id_to_endpoint = HashMap::<ResourceId, EndPoint>::new();
        for ep in it_src {
            let opt_resource_id = resource_id_retriever.retrieve(ep);

            if let Ok(resource_id) = opt_resource_id {
                resource_id_to_endpoint.insert(resource_id, ep.clone());
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", ep);
            }
        }
        Self {
            resource_id_to_endpoint: Rc::new(resource_id_to_endpoint),
        }
    }
}

impl EndpointRetriever for EndpointResourceIdMap {
    fn retrieve(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::EndPoint;
    use super::EndpointResourceIdMap;
    use crate::resources::endpoint_retriever::EndpointRetriever;
    use crate::resources::resource_id_retriever::MockResourceIdResolver;
    use crate::types::ResourceId;
    use std::path::PathBuf;

    fn create_dut(test_ep_list: Vec<EndPoint>) -> EndpointResourceIdMap {
        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_retrieve()
            .withf(|f| matches!(f, EndPoint::FileUnknown(path) if path==&PathBuf::from("testpath")))
            .returning(|_f| Ok(ResourceId("[[testpath]]".to_string())));

        let dut = EndpointResourceIdMap::new(test_ep_list.iter(), &mock_res_id_res);
        dut
    }
    #[test]
    fn test_resolve_single_entry() {
        let dut = create_dut(vec![EndPoint::FileUnknown("testpath".into())]);
        let ep = dut.retrieve(&"[[testpath]]".into()).unwrap();

        assert!(matches!(ep, EndPoint::FileUnknown(path) if path==PathBuf::from("testpath")));
    }

    #[test]
    fn test_new_correct_iteration() {
        let test_ep_list: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpath".into())];

        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_retrieve()
            .returning(|_f| Ok(ResourceId("[[doesnt matter]]".to_string())));

        let _dut = EndpointResourceIdMap::new(test_ep_list.iter(), &mock_res_id_res);
    }

    #[test]
    fn test_new_resolve() {
        let test_ep_list: Vec<EndPoint> = vec![EndPoint::FileUnknown("testpath".into())];

        let mut mock_res_id_res = MockResourceIdResolver::new();
        mock_res_id_res
            .expect_retrieve()
            .withf(|f| matches!(f, EndPoint::FileUnknown(path) if path==&PathBuf::from("testpath")))
            .returning(|_f| Ok(ResourceId("[[doesnt matter]]".to_string())));

        let _dut = EndpointResourceIdMap::new(test_ep_list.iter(), &mock_res_id_res);
    }
}
