use crate::resources::content_retriever::MdContentRetriever;
use crate::types::{Content, ResourceId};
use crate::Result;

pub fn trafo_resource_ids_to_content<'a>(
    iter: impl Iterator<Item = &'a ResourceId> + 'a,
    content_retriever: &'a impl MdContentRetriever,
) -> impl Iterator<Item = (ResourceId, Result<Content>)> + 'a {
    // load content.
    // iterator yields (ResourceId, Result<Content>)
    iter.map(move |f| {
        (
            f.clone(),
            content_retriever.retrieve(&f).map(|f| f.to_owned()),
        )
    })
}
