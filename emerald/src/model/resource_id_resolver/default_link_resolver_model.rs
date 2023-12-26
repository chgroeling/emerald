use super::adapter_to_resource_loc::adapter_to_resourece_loc;
use super::resource_id_link_map::ResourceIdLinkMap;
use super::resource_id_resolver_trait::ResourceIdResolver;
use crate::types;
use std::path::Path;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct DefaultResourceIdResolverModel {
    link_map: ResourceIdLinkMap,
}

impl DefaultResourceIdResolverModel {
    pub fn new<'a>(
        it_src: impl IntoIterator<Item = &'a (types::ResourceId, types::FilesystemMetadata)> + 'a,
        common_path: &'a Path,
    ) -> Self {
        let resource_loc_iter = adapter_to_resourece_loc(it_src, common_path);
        Self {
            link_map: ResourceIdLinkMap::new(resource_loc_iter),
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
