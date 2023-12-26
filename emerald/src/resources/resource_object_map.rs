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
        it_src: impl IntoIterator<Item = &'a (&'a ResourceObject, types::ResourceId)>,
    ) -> Self {
        let mut rid_to_ro = HashMap::<types::ResourceId, ResourceObject>::new();
        for (ro, res_id) in it_src {
            if rid_to_ro.insert(res_id.clone(), (*ro).clone()).is_some() {
                panic!("Resource Ids must be unique!")
            }
        }
        Self {
            rid_to_ro: Rc::new(rid_to_ro),
        }
    }
}

impl ResourceObjectRetriever for ResourceObjectMap {
    fn retrieve(&self, rid: &types::ResourceId) -> Result<ResourceObject> {
        self.rid_to_ro
            .get(rid)
            .map_or(Err(ResourceObjectNotFound), |f| Ok(f.clone()))
    }
}
