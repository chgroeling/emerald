use std::rc::Rc;

pub mod endpoint_resource_id_map;
mod link_querier;
mod resource_id_link_map;
pub mod resource_id_querier;
mod src_iter_querier;
mod src_links_map;
mod tgt_iter_querier;
mod tgt_links_map;

pub use self::link_querier::LinkQuerier;
pub use self::src_iter_querier::SrcIterQuerier;
pub use self::tgt_iter_querier::TgtIterQuerier;

use self::{
    resource_id_link_map::ResourceIdLinkMap, src_links_map::SrcLinksMap, tgt_links_map::TgtLinksMap,
};
use crate::indexes::{ResourceIdsIterSrc, Src2TgtIterSrc};

pub fn create_link_querier(resource_ids_iter_rc: &impl ResourceIdsIterSrc) -> Rc<dyn LinkQuerier> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iter_rc))
}

pub fn create_tgt_iter_querier(src_tgt_iter_rc: &impl Src2TgtIterSrc) -> Rc<dyn TgtIterQuerier> {
    Rc::new(TgtLinksMap::new(src_tgt_iter_rc))
}

pub fn create_src_iter_querier(src_tgt_iter_rc: &impl Src2TgtIterSrc) -> Rc<dyn SrcIterQuerier> {
    Rc::new(SrcLinksMap::new(src_tgt_iter_rc))
}
