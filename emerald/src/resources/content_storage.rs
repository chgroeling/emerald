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
type ResourceIdToContentIdx = HashMap<ResourceId, Content>;

pub struct ContentStorage {
    res_id_to_content: ResourceIdToContentIdx,
    res_id_to_content_list: ResourceIdContentList,
}

impl<'a> ContentStorage {
    pub fn new(
        md_resource_ids_iter_src: &impl MdResourceIdsIterSource,
        content_loader: &'a impl ContentLoader,
    ) -> ContentStorage {
        let mut res_id_to_content_list = ResourceIdContentList::new();
        let mut res_id_to_content_idx = ResourceIdToContentIdx::new();

        for md_res_id in md_resource_ids_iter_src.md_iter() {
            let read_note = content_loader.load(md_res_id.clone());

            // ignore files that cannot be read
            if let Ok(content) = read_note {
                trace!("Loaded {:?} into string", &md_res_id);

                // insert actual index into hashmap
                res_id_to_content_idx.insert(md_res_id.clone(), content.clone());
                res_id_to_content_list.push((md_res_id, content));
            } else {
                warn!("File {:?} could not be loaded", &md_res_id)
            }
        }

        Self {
            res_id_to_content_list,
            res_id_to_content: res_id_to_content_idx,
        }
    }
}

impl ContentQueryable for ContentStorage {
    fn get(&self, resource_id: ResourceId) -> Option<Content> {
        Some(self.res_id_to_content.get(&resource_id)?.clone())
    }
}

impl ContentIterSource for ContentStorage {
    type Iter = std::vec::IntoIter<(ResourceId, Content)>;
    fn iter(&self) -> Self::Iter {
        self.res_id_to_content_list.clone().into_iter()
    }
}
