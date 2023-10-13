use crate::resources::md_content_retriever::MdContentRetriever;
use crate::types::{Content, ResourceId};
use crate::Result;

pub fn trafo_resource_ids_to_content<'a>(
    md_res_id_iter: impl Iterator<Item = &'a ResourceId>,
    content_retriever: &'a impl MdContentRetriever,
) -> impl Iterator<Item = (ResourceId, Result<&'a Content>)> {
    // load content.
    // iterator yields (ResourceId, Result<Content>)
    md_res_id_iter.map(move |f| (f.clone(), content_retriever.retrieve(f)))
}