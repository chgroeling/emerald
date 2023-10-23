use super::endpoint_retriever::EndpointRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct EndpointResourceIdMap {
    resource_id_to_endpoint: Rc<HashMap<types::ResourceId, types::ResourceObject>>,
}

impl EndpointResourceIdMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::ResourceObject, types::ResourceId)>,
    ) -> Result<Self> {
        let mut resource_id_to_endpoint =
            HashMap::<types::ResourceId, types::ResourceObject>::new();
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
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<types::ResourceObject> {
        self.resource_id_to_endpoint
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}
