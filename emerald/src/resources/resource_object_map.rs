use super::resource_object::ResourceObject;
use super::resource_object_retriever::ResourceObjectRetriever;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct ResourceObjectMap {
    rid_to_ro: Rc<HashMap<types::ResourceId, ResourceObject>>,
}

impl ResourceObjectMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (ResourceObject, types::ResourceId)>,
    ) -> Result<Self> {
        let mut resource_id_to_endpoint = HashMap::<types::ResourceId, ResourceObject>::new();
        for (ro, res_id) in it_src {
            if resource_id_to_endpoint
                .insert(res_id.clone(), ro.clone())
                .is_some()
            {
                return Err(NotUnique);
            }
        }
        Ok(Self {
            rid_to_ro: Rc::new(resource_id_to_endpoint),
        })
    }
}

impl ResourceObjectRetriever for ResourceObjectMap {
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<ResourceObject> {
        self.rid_to_ro
            .get(resource_id)
            .map_or(Err(EndPointNotFound), |f| Ok(f.clone()))
    }
}
