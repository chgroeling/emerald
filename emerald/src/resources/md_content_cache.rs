use super::{content_loader::ContentLoader, md_content_retriever::MdContentRetriever};
use crate::types;
use crate::Result;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct MdContentCache {
    res_id_to_content: Rc<HashMap<types::ResourceId, types::Content>>,
}

impl MdContentCache {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a types::ResourceId>,
        content_loader: &'a impl ContentLoader,
    ) -> Self {
        let mut res_id_to_content = HashMap::<types::ResourceId, types::Content>::new();

        for md_res_id in it_src.into_iter() {
            let read_note = content_loader.load(md_res_id);

            // ignore files that cannot be read
            if let Ok(content) = read_note {
                trace!("Loaded {:?} into string", md_res_id);

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
    fn retrieve(&self, resource_id: &types::ResourceId) -> Result<&types::Content> {
        let cached = self.res_id_to_content.get(resource_id);

        match cached {
            Some(entry) => Ok(entry),
            _ => Err(crate::EmeraldError::NotAMarkdownFile),
        }
    }
}
