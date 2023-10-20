use crate::{EmeraldError, Result};
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
    pub fn new<'a>(it_src: impl IntoIterator<Item = &'a (EndPoint, ResourceId)>) -> Result<Self> {
        let mut resource_id_to_endpoint = HashMap::<ResourceId, EndPoint>::new();
        for (ep, res_id) in it_src {
            if resource_id_to_endpoint
                .insert(res_id.clone(), ep.clone())
                .is_some()
            {
                return Err(NotUnique);
            }
        }
        Ok(Self {
            resource_id_to_endpoint: Rc::new(resource_id_to_endpoint),
        })
    }
}

impl EndpointRetriever for EndpointResourceIdMap {
    fn retrieve(&self, resource_id: &ResourceId) -> Result<EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}
