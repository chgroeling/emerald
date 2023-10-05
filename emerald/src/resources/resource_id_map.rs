use crate::{resources::endpoints_iter_src::EndpointsIterSrc, EmeraldError, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, path::Path};

use crate::{
    types::{EndPoint, ResourceId},
    utils::endpoint_translation::convert_endpoint_to_resource_id,
};
use EmeraldError::*;

use super::resource_id_getter::ResourceIdGetter;

pub struct ResourceIdMap {
    ep_to_resource_id: HashMap<EndPoint, ResourceId>,
}

impl ResourceIdMap {
    pub fn new(ep_iter_rc: &impl EndpointsIterSrc, common_path: &Path) -> Self {
        let mut ep_to_resource_id = HashMap::<EndPoint, ResourceId>::new();
        for ep in ep_iter_rc.iter() {
            let opt_resource_id = convert_endpoint_to_resource_id(ep.clone(), common_path);

            if let Some(resource_id) = opt_resource_id {
                ep_to_resource_id.insert(ep, resource_id);
            } else {
                warn!("Can't convert Endpoint '{:?}' to ResourceId.", &ep);
            }
        }
        Self { ep_to_resource_id }
    }
}

impl ResourceIdGetter for ResourceIdMap {
    fn get(&self, ep: &EndPoint) -> Result<ResourceId> {
        self.ep_to_resource_id
            .get(ep)
            .map_or(Err(ResourceIdNotFound), |f| Ok(f.clone()))
    }
}

#[cfg(test)]
mod tests {}
