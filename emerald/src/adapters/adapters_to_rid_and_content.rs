use crate::error::Result;
use crate::{resources, types};

pub fn adapter_from_rids_to_rids_and_content<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId>,
    content_retriever: &'a impl resources::MdContentRetriever,
) -> Result<impl Iterator<Item = (&'a types::ResourceId, &'a types::Content)>> {
    // load content into Iterator. Iterator yields (ResourceId, Result<Content>)

    let ret: Result<Vec<_>> = it_src
        .into_iter()
        .map(move |f| content_retriever.retrieve(f).map(|op| (f, op)))
        .collect();

    match ret {
        Ok(vector) => Ok(vector.into_iter()),
        Err(err) => Err(err),
    }
}
