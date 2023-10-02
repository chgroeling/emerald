use crate::{EmeraldError, Result};
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::{content_iterable::ContentIterable, content_loader::ContentLoader};
use crate::{
    indexes::ResourceIdsIterable,
    types::{Content, ResourceId},
};

pub struct ContentFullCache {
    res_id_to_content: HashMap<ResourceId, Content>,
    res_id_to_content_vec: Vec<(ResourceId, Content)>,
}

impl<'a> ContentFullCache {
    pub fn new(
        md_resource_ids_iterable: &impl ResourceIdsIterable,
        content_loader: &'a impl ContentLoader,
    ) -> ContentFullCache {
        let mut res_id_to_content_list = Vec::<(ResourceId, Content)>::new();
        let mut res_id_to_content_idx = HashMap::<ResourceId, Content>::new();

        for md_res_id in md_resource_ids_iterable.iter() {
            let read_note = content_loader.load(&md_res_id);

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
            res_id_to_content_vec: res_id_to_content_list,
            res_id_to_content: res_id_to_content_idx,
        }
    }
}

impl ContentLoader for ContentFullCache {
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        self.res_id_to_content
            .get(resource_id)
            .ok_or(EmeraldError::ResourceIdNotFound)
            .cloned()
    }
}

impl ContentIterable for ContentFullCache {
    type Iter = std::vec::IntoIter<(ResourceId, Content)>;
    fn iter(&self) -> Self::Iter {
        self.res_id_to_content_vec.clone().into_iter()
    }
}
