use crate::resources::md_content_retriever::MdContentRetriever;
use crate::types::{Content, ResourceId};
use crate::Result;

pub fn trafo_from_res_ids_to_content<'a>(
    it_src: impl IntoIterator<Item = &'a ResourceId>,
    content_retriever: &'a impl MdContentRetriever,
) -> impl Iterator<Item = (&'a ResourceId, Result<&'a Content>)> {
    // load content into Iterator. Iterator yields (ResourceId, Result<Content>)
    it_src
        .into_iter()
        .map(move |f| (f, content_retriever.retrieve(f)))
}
