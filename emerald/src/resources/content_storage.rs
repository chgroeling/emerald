use std::collections::HashMap;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::{
    content_iter_source::ContentIterSource, content_loader::ContentLoader,
    content_queryable::ContentQueryable,
};
use crate::{
    indexes::MdResourceIdsIterSource,
    types::{Content, ResourceId},
};

type ResourceIdContentList = Vec<(ResourceId, Content)>;
type ResourceIdToContentIdx = HashMap<ResourceId, usize>;

pub struct ContentStorage {
    resource_to_content_idx: ResourceIdToContentIdx,
    ep_content_list: ResourceIdContentList,
}

impl<'a> ContentStorage {
    pub fn new(
        md_resource_ids_iter_src: &impl MdResourceIdsIterSource,
        content_loader: &'a impl ContentLoader,
    ) -> ContentStorage {
        let mut ep_content_list = ResourceIdContentList::new();
        let mut resource_id_to_content_idx = ResourceIdToContentIdx::new();

        for md_res_id in md_resource_ids_iter_src.md_iter() {
            let read_note = content_loader.load(md_res_id.clone());

            // ignore files that cannot be read
            if let Ok(content) = read_note {
                trace!("Loaded {:?} into string", &md_res_id);

                // insert actual index into hashmap
                resource_id_to_content_idx.insert(md_res_id.clone(), ep_content_list.len());
                ep_content_list.push((md_res_id, content));
            } else {
                warn!("File {:?} could not be loaded", &md_res_id)
            }
        }

        Self {
            ep_content_list,
            resource_to_content_idx: resource_id_to_content_idx,
        }
    }
}

impl ContentQueryable for ContentStorage {
    fn get(&self, resource_id: &ResourceId) -> Option<Content> {
        let content_idx = self.resource_to_content_idx.get(resource_id)?;
        let ret: Content;
        unsafe {
            ret = self.ep_content_list.get_unchecked(*content_idx).1.clone();
        }
        Some(ret)
    }
}

impl ContentIterSource for ContentStorage {
    type Iter = std::vec::IntoIter<(ResourceId, Content)>;
    fn iter(&self) -> Self::Iter {
        self.ep_content_list.clone().into_iter()
    }
}
