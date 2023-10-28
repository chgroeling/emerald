use crate::{model::content, types};

pub fn adapter_to_rids_and_content<'a>(
    it_src: impl IntoIterator<Item = &'a types::ResourceId>,
    content_retriever: &'a impl content::MdContentRetriever,
) -> impl Iterator<Item = (&'a types::ResourceId, &'a types::Content)> {
    // load content into Iterator. Iterator yields (ResourceId, Result<Content>)

    it_src
        .into_iter()
        .map(move |f| (f, content_retriever.retrieve(f)))
}
