use super::md_content_map::MdContentMap;
use super::md_content_retriever::MdContentRetriever;
use crate::types;

#[derive(Clone)]
pub struct DefaultContentModel {
    md_content_map: MdContentMap,
}

impl DefaultContentModel {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, types::Content)>) -> Self {
        Self {
            md_content_map: MdContentMap::new(it_src),
        }
    }
}

impl MdContentRetriever for DefaultContentModel {
    fn retrieve(&self, rid: &types::ResourceId) -> &types::Content {
        self.md_content_map.retrieve(rid)
    }
}
