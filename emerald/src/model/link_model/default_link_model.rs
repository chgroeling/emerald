use super::resource_id_link_map::ResourceIdLinkMap;
use super::resource_id_resolver::ResourceIdResolver;
use crate::types;

pub struct DefaultLinkModel {
    link_map: ResourceIdLinkMap,
}

impl DefaultLinkModel {
    pub fn new<'a>(it_src: impl IntoIterator<Item = (&'a types::ResourceId, String)>) -> Self {
        Self {
            link_map: ResourceIdLinkMap::new(it_src),
        }
    }
}

impl ResourceIdResolver for DefaultLinkModel {
    fn resolve_with_hint(
        &self,
        link: &types::Link,
        hint: super::resource_id_resolver::Hint,
    ) -> crate::Result<types::ResourceId> {
        self.link_map.resolve_with_hint(link, hint)
    }
}
