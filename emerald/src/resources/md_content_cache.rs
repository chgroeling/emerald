use crate::Result;
use std::{collections::HashMap, rc::Rc};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::{content_loader::ContentLoader, content_retriever::MdContentRetriever};
use crate::types::{Content, ResourceId};

#[derive(Clone)]
pub struct MdContentCache {
    res_id_to_content: Rc<HashMap<ResourceId, Content>>,
}

impl MdContentCache {
    pub fn new<'a>(
        md_resource_ids_iter: impl Iterator<Item = &'a ResourceId>,
        content_loader: &impl ContentLoader,
    ) -> Self {
        let mut res_id_to_content = HashMap::<ResourceId, Content>::new();

        for md_res_id in md_resource_ids_iter {
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
        }
    }
}

impl MdContentRetriever for MdContentCache {
    fn retrieve(&self, resource_id: &ResourceId) -> Result<&Content> {
        let cached = self.res_id_to_content.get(resource_id);

        match cached {
            Some(entry) => Ok(entry),
            _ => panic!(
                "MdContentCache has not cached resource_id={:?}.",
                resource_id
            ),
        }
    }
}
