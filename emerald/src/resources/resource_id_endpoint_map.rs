use crate::{EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

use crate::types::{EndPoint, ResourceId};
use EmeraldError::*;

use super::resource_id_retriever::ResourceIdRetriever;

#[derive(Clone)]
pub struct ResourceIdEndPointMap {
    ep_to_resource_id: Rc<HashMap<EndPoint, ResourceId>>,
}

impl ResourceIdEndPointMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = &'a (EndPoint, ResourceId)>) -> Result<Self> {
        let mut ep_to_resource_id = HashMap::<EndPoint, ResourceId>::new();
        for (ep, res_id) in it_src.into_iter() {
            if ep_to_resource_id
                .insert(ep.clone(), res_id.clone())
                .is_some()
            {
                return Err(NotUnique);
            }
        }
        Ok(Self {
            ep_to_resource_id: Rc::new(ep_to_resource_id),
        })
    }
}

impl ResourceIdRetriever for ResourceIdEndPointMap {
    fn retrieve(&self, ep: &EndPoint) -> Result<ResourceId> {
        self.ep_to_resource_id
            .get(ep)
            .map_or(Err(EndpointHasNoResourceId(ep.clone())), |f| Ok(f.clone()))
    }
}
