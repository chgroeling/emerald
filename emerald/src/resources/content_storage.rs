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

type EndPointContentList = Vec<(ResourceId, Content)>;
type EndPointLinkToContentIdx = HashMap<ResourceId, usize>;

pub struct ContentStorage {
    resource_to_content_idx: EndPointLinkToContentIdx,
    ep_content_list: EndPointContentList,
}

impl<'a> ContentStorage {
    pub fn new(
        ep_md_iter_src: &impl MdResourceIdsIterSource,
        content_loader: &'a impl ContentLoader,
    ) -> ContentStorage {
        let mut ep_content_list = EndPointContentList::new();
        let mut resource_id_to_content_idx = EndPointLinkToContentIdx::new();

        for ep_md_link in ep_md_iter_src.md_iter() {
            let read_note = content_loader.load(&ep_md_link);

            // ignore files that cannot be read
            if let Ok(content) = read_note {
                trace!("Loaded {:?} into string", &ep_md_link);

                // insert actual index into hashmap
                resource_id_to_content_idx.insert(ep_md_link.clone(), ep_content_list.len());
                ep_content_list.push((ep_md_link, content));
            } else {
                warn!("File {:?} could not be loaded", &ep_md_link)
            }
        }

        Self {
            ep_content_list,
            resource_to_content_idx: resource_id_to_content_idx,
        }
    }
}

impl ContentQueryable for ContentStorage {
    fn get(&self, resource_id: &ResourceId) -> Option<&Content> {
        let content_idx = self.resource_to_content_idx.get(resource_id)?;
        let ret: &Content;
        unsafe {
            ret = &self.ep_content_list.get_unchecked(*content_idx).1;
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
