use super::md_content_retriever::MdContentRetriever;
use crate::error::EmeraldError;
use crate::error::Result;
use crate::resources;
use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct MdContentMap {
    res_id_to_content: Rc<HashMap<types::ResourceId, types::Content>>,
}

pub fn adapter_to_rid_and_content<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId>,
    content_loader: &'a impl resources::ContentLoader,
) -> Result<impl Iterator<Item = (types::ResourceId, types::Content)>> {
    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(|rid| -> Result<(types::ResourceId, types::Content)> {
            let content = content_loader.load(rid)?;
            trace!("Loaded {:?} into string", rid);
            Ok((rid.clone(), content))
        })
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}

impl MdContentMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = (types::ResourceId, types::Content)>) -> Self {
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
    fn retrieve(&self, rid: &types::ResourceId) -> Result<&types::Content> {
        let cached = self.res_id_to_content.get(rid);

        match cached {
            Some(entry) => Ok(entry),
            _ => Err(crate::EmeraldError::NotAMarkdownFile),
        }
    }
}
