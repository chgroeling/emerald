use super::resource_id_retriever::ResourceIdRetriever;
use super::resource_object::ResourceObject;
use crate::error::{EmeraldError::*, Result};
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct ResourceIdMap {
    ro_to_rid: Rc<HashMap<ResourceObject, types::ResourceId>>,
}

impl ResourceIdMap {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (ResourceObject, types::ResourceId)>,
    ) -> Self {
        let mut ro_to_rid = HashMap::<ResourceObject, types::ResourceId>::new();
        for (ro, res_id) in it_src.into_iter() {
            if ro_to_rid.insert(ro.clone(), res_id.clone()).is_some() {
                panic!("Resource Objects must be unique!");
            }
        }
        Self {
            ro_to_rid: Rc::new(ro_to_rid),
        }
    }
}

impl ResourceIdRetriever for ResourceIdMap {
    fn retrieve(&self, ro: &ResourceObject) -> Result<types::ResourceId> {
        self.ro_to_rid
            .get(ro)
            .map_or(Err(ResourceObjectHasNoResourceId(format!("{ro:?}"))), |f| {
                Ok(f.clone())
            })
    }
}
