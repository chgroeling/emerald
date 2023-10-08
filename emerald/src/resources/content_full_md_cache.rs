use crate::Result;
use std::{collections::HashMap, rc::Rc};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_loader::ContentLoader;
use crate::{
    indexes::ResourceIdsIterSrc,
    types::{Content, ResourceId},
};

#[derive(Clone)]
pub struct ContentFullMdCache<I>
where
    I: ContentLoader,
{
    res_id_to_content: Rc<HashMap<ResourceId, Content>>,
    content_loader: I,
}

impl<I> ContentFullMdCache<I>
where
    I: ContentLoader,
{
    pub fn new(md_resource_ids_iter_rc: &impl ResourceIdsIterSrc, content_loader: I) -> Self {
        let mut res_id_to_content = HashMap::<ResourceId, Content>::new();

        for md_res_id in md_resource_ids_iter_rc.iter() {
            let read_note = content_loader.load(&md_res_id);

            // ignore files that cannot be read
            if let Ok(content) = read_note {
                trace!("Loaded {:?} into string", &md_res_id);

                // insert actual index into hashmap
                res_id_to_content.insert(md_res_id.clone(), content.clone());
            } else {
                warn!("File {:?} could not be loaded", &md_res_id)
            }
        }

        Self {
            res_id_to_content: Rc::new(res_id_to_content),
            content_loader,
        }
    }
}

impl<I> ContentLoader for ContentFullMdCache<I>
where
    I: ContentLoader,
{
    fn load(&self, resource_id: &ResourceId) -> Result<Content> {
        let cached = self.res_id_to_content.get(resource_id);

        match cached {
            Some(entry) => Ok(entry.clone()),
            _ => self.content_loader.load(resource_id),
        }
    }
}
