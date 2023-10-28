use crate::{resources, types};

use super::{md_content_cache::MdContentCache, MdContentRetriever};

#[derive(Clone)]
pub struct DefaultContentModel {
    content_map: MdContentCache,
}

impl DefaultContentModel {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a types::ResourceId>,
        content_loader: &'a impl resources::ContentLoader,
    ) -> Self {
        Self {
            content_map: MdContentCache::new(it_src, content_loader),
        }
    }
}

impl MdContentRetriever for DefaultContentModel {
    fn retrieve(&self, rid: &types::ResourceId) -> crate::Result<&types::Content> {
        self.content_map.retrieve(rid)
    }
}
