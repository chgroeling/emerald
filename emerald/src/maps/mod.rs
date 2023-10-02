use std::rc::Rc;

pub mod endpoint_resource_id_map;
pub mod endpoint_retriever;
mod resource_id_link_map;
mod resource_id_retriever;
mod src_iter_retriever;
mod src_links_map;
mod tgt_iter_retriever;
mod tgt_links_map;

pub use self::resource_id_retriever::ResourceIdRetriever;
pub use self::src_iter_retriever::SrcIterRetriever;
pub use self::tgt_iter_retriever::TgtIterRetriever;

use self::{
    resource_id_link_map::ResourceIdLinkMap, src_links_map::SrcLinksMap, tgt_links_map::TgtLinksMap,
};
use crate::indexes::{ResourceIdsIterSrc, Src2TgtIterSrc};

pub fn create_link_retriever(
    resource_ids_iter_rc: &impl ResourceIdsIterSrc,
) -> Rc<dyn ResourceIdRetriever> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iter_rc))
}

pub fn create_tgt_iter_retriever(
    src_tgt_iter_rc: &impl Src2TgtIterSrc,
) -> Rc<dyn TgtIterRetriever> {
    Rc::new(TgtLinksMap::new(src_tgt_iter_rc))
}

pub fn create_src_iter_retriever(
    src_tgt_iter_rc: &impl Src2TgtIterSrc,
) -> Rc<dyn SrcIterRetriever> {
    Rc::new(SrcLinksMap::new(src_tgt_iter_rc))
}
