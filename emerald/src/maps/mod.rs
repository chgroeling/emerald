use std::rc::Rc;

mod link_queryable;
mod resource_id_link_map;
mod src_iter_queryable;
mod src_links_map;
mod tgt_iter_queryable;
mod tgt_links_map;

pub use self::link_queryable::LinkQueryable;
pub use self::src_iter_queryable::SrcIterQueryable;
pub use self::tgt_iter_queryable::TgtIterQueryable;

use self::{
    resource_id_link_map::ResourceIdLinkMap, src_links_map::SrcLinksMap, tgt_links_map::TgtLinksMap,
};
use crate::indexes::{ResourceIdsIterable, Src2TgtIterable};

pub fn create_link_queryable(
    resource_ids_iterable: &impl ResourceIdsIterable,
) -> Rc<dyn LinkQueryable> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iterable))
}

pub fn create_tgt_iter_queryable(
    src_tgt_iterable: &impl Src2TgtIterable,
) -> Rc<dyn TgtIterQueryable> {
    Rc::new(TgtLinksMap::new(src_tgt_iterable))
}

pub fn create_src_iter_queryable(
    src_tgt_iterable: &impl Src2TgtIterable,
) -> Rc<dyn SrcIterQueryable> {
    Rc::new(SrcLinksMap::new(src_tgt_iterable))
}
