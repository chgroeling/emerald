use crate::{resources, types};

use super::{md_content_map::MdContentMap, MdContentRetriever};

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
    fn retrieve(&self, rid: &types::ResourceId) -> crate::Result<&types::Content> {
        self.content_map.retrieve(rid)
    }
}
