use super::resource_id_retriever::ResourceIdRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct ResourceIdEndPointMap {
    ep_to_resource_id: Rc<HashMap<types::EndPoint, types::ResourceId>>,
}

impl ResourceIdEndPointMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::EndPoint, types::ResourceId)>,
    ) -> Result<Self> {
        let mut ep_to_resource_id = HashMap::<types::EndPoint, types::ResourceId>::new();
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
    fn retrieve(&self, ep: &types::EndPoint) -> Result<types::ResourceId> {
        self.ep_to_resource_id
            .get(ep)
            .map_or(Err(EndpointHasNoResourceId(ep.clone())), |f| Ok(f.clone()))
    }
}
