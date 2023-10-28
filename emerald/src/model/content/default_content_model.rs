use super::md_content_map::MdContentMap;
use super::md_content_retriever::MdContentRetriever;
use crate::types;

#[derive(Clone)]
pub struct DefaultContentModel {
    content_map: MdContentMap,
}

impl DefaultContentModel {
    pub fn new<'a>(it_src: impl IntoIterator<Item = (types::ResourceId, types::Content)>) -> Self {
        Self {
            content_map: MdContentMap::new(it_src),
        }
    }
}

impl MdContentRetriever for DefaultContentModel {
    fn retrieve(&self, rid: &types::ResourceId) -> &types::Content {
        self.content_map.retrieve(rid)
    }
}
