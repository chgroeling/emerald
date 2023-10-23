use super::resource_id_retriever::ResourceIdRetriever;
use super::resource_object::ResourceObject;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct ResourceIdEndPointMap {
    ep_to_resource_id: Rc<HashMap<ResourceObject, types::ResourceId>>,
}

impl ResourceIdEndPointMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (ResourceObject, types::ResourceId)>,
    ) -> Result<Self> {
        let mut ep_to_resource_id = HashMap::<ResourceObject, types::ResourceId>::new();
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
    fn retrieve(&self, ep: &ResourceObject) -> Result<types::ResourceId> {
        self.ep_to_resource_id
            .get(ep)
            .map_or(Err(EndpointHasNoResourceId(format!("{ep:?}"))), |f| {
                Ok(f.clone())
            })
    }
}
