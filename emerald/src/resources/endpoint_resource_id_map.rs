use crate::types;
use crate::{EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};
use EmeraldError::*;

use super::endpoint_retriever::EndpointRetriever;

#[derive(Clone)]
pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: Rc<HashMap<types::ResourceId, types::EndPoint>>,
}

impl EndpointResourceIdMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::EndPoint, types::ResourceId)>,
    ) -> Result<Self> {
        let mut resource_id_to_endpoint = HashMap::<types::ResourceId, types::EndPoint>::new();
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
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<types::EndPoint> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}
