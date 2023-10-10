use self::{
    convert_to_link_2_tgt::convert_to_link_2_tgt,
    convert_to_link_src_2_tgt::convert_to_link_src_2_tgt,
    extract_content_types::extract_content_types, extract_links::extract_links,
};
use crate::Result;
use crate::{
    maps::ResourceIdRetriever,
    resources::content_loader::ContentLoader,
    types::{LinkSrc2Tgt, ResourceId},
};
mod content_type;
mod convert_to_link_2_tgt;
mod convert_to_link_src_2_tgt;
mod extract_content_types;
mod extract_links;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn extract_links_from_vault<'a>(
    iter: impl Iterator<Item = ResourceId> + 'static,
    content_loader: &'a impl ContentLoader,
    resource_id_retriever: &'a impl ResourceIdRetriever,
) -> impl Iterator<Item = (ResourceId, Result<Vec<LinkSrc2Tgt>>)> + 'a {
    // load content.
    // iterator yields (ResourceId, Result<Content>)
    let content_iter = iter.map(move |f| (f.clone(), content_loader.load(&f)));

    // iterator yield (a, b)
    // a: the resource id of the source which was loaded
    // b: a vector containing the links which were found wrapped in a Result
    let all_links_iter = content_iter.map(move |f| {
        (
            f.0.clone(),
            f.1.map(move |content| {
                trace!("Link extraction from {:?} starts", &f.0);

                let content_type_iter = extract_content_types(content);
                let link_iter = extract_links(content_type_iter);
                let link_2_tgt_iter = convert_to_link_2_tgt(link_iter, resource_id_retriever);
                convert_to_link_src_2_tgt(f.0, link_2_tgt_iter).collect()
            }),
        )
    });

    all_links_iter
}
