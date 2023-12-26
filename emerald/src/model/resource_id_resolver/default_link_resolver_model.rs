use std::path::Path;

use super::resource_id_link_map::ResourceIdLinkMap;
use super::resource_id_resolver_trait::ResourceIdResolver;
use crate::types;

pub struct DefaultResourceIdResolverModel {
    link_map: ResourceIdLinkMap,
}

impl DefaultResourceIdResolverModel {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)> + 'a,
        common_path: &'a Path,
    ) -> Self {
        Self {
            link_map: ResourceIdLinkMap::new(it_src, common_path),
        }
    }
}

impl ResourceIdResolver for DefaultResourceIdResolverModel {
    fn resolve_with_hint(
        &self,
        link: &types::Link,
        hint: super::resource_id_resolver_trait::Hint,
    ) -> crate::Result<types::ResourceId> {
        self.link_map.resolve_with_hint(link, hint)
    }
}
