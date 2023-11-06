use super::resource_id_link_map::ResourceIdLinkMap;
use super::resource_id_resolver::ResourceIdResolver;
use crate::types;

pub struct DefaultResourceIdResolverModel {
    link_map: ResourceIdLinkMap,
}

impl DefaultResourceIdResolverModel {
    pub fn new(it_src: impl IntoIterator<Item = (types::ResourceId, String)>) -> Self {
        Self {
            link_map: ResourceIdLinkMap::new(it_src),
        }
    }
}

impl ResourceIdResolver for DefaultResourceIdResolverModel {
    fn resolve_with_hint(
        &self,
        link: &types::Link,
        hint: super::resource_id_resolver::Hint,
    ) -> crate::Result<types::ResourceId> {
        self.link_map.resolve_with_hint(link, hint)
    }
}
