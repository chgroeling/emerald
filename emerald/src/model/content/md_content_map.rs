use super::md_content_retriever::MdContentRetriever;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct MdContentMap {
    res_id_to_content: Rc<HashMap<types::ResourceId, types::Content>>,
}

impl MdContentMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = (types::ResourceId, types::Content)>) -> Self {
        // I assume that all resource ids are existent
        let mut res_id_to_content = HashMap::<types::ResourceId, types::Content>::new();

        for (res_id, content) in it_src.into_iter() {
            if let Some(_) = res_id_to_content.insert(res_id, content) {
                panic!("Unique resource ids required")
            }
        }

        Self {
            res_id_to_content: Rc::new(res_id_to_content),
        }
    }
}

impl MdContentRetriever for MdContentMap {
    fn retrieve(&self, rid: &types::ResourceId) -> &types::Content {
        let cached = self.res_id_to_content.get(rid);

        match cached {
            Some(entry) => entry,
            _ => panic!("This should not happen. Requested non existant resource id."),
        }
    }
}
