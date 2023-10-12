use crate::Result;
use crate::{
    resources::content_loader::ContentLoader,
    types::{Content, ResourceId},
};

pub fn trafo_resource_ids_to_content<'a>(
    iter: impl Iterator<Item = &'a ResourceId> + 'a,
    content_loader: &'a impl ContentLoader,
) -> impl Iterator<Item = (ResourceId, Result<Content>)> + 'a {
    // load content.
    // iterator yields (ResourceId, Result<Content>)
    iter.map(move |f| (f.clone(), content_loader.load(&f)))
}
