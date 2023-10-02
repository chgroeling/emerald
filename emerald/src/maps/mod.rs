use std::rc::Rc;

pub mod endpoint_resource_id_map;
mod link_queryable;
mod resource_id_link_map;
pub mod resource_id_queryable;
mod src_iter_queryable;
mod src_links_map;
mod tgt_iter_queryable;
mod tgt_links_map;

pub use self::link_queryable::LinkQuerier;
pub use self::src_iter_queryable::SrcIterQuerier;
pub use self::tgt_iter_queryable::TgtIterQuerier;

use self::{
    resource_id_link_map::ResourceIdLinkMap, src_links_map::SrcLinksMap, tgt_links_map::TgtLinksMap,
};
use crate::indexes::{ResourceIdsIterSrc, Src2TgtIterSrc};

pub fn create_link_queryable(
    resource_ids_iter_rc: &impl ResourceIdsIterSrc,
) -> Rc<dyn LinkQuerier> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iter_rc))
}

pub fn create_tgt_iter_queryable(src_tgt_iter_rc: &impl Src2TgtIterSrc) -> Rc<dyn TgtIterQuerier> {
    Rc::new(TgtLinksMap::new(src_tgt_iter_rc))
}

pub fn create_src_iter_queryable(src_tgt_iter_rc: &impl Src2TgtIterSrc) -> Rc<dyn SrcIterQuerier> {
    Rc::new(SrcLinksMap::new(src_tgt_iter_rc))
}
