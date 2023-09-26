use std::rc::Rc;

mod link_queryable;
mod resource_id_link_map;
mod src_iter_queryable;
mod src_list_map;
mod target_iterator_queryable;
mod target_list_map;

pub use self::link_queryable::LinkQueryable;
pub use self::src_iter_queryable::SrcIterQueryable;
pub use self::target_iterator_queryable::TargetIteratorQueryable;

use self::{
    resource_id_link_map::ResourceIdLinkMap, src_list_map::SrcListMap,
    target_list_map::TargetListMap,
};
use crate::indexes::{ResourceIdsIterable, SrcTgtIterable};

pub fn create_link_queryable(
    resource_ids_iterable: &impl ResourceIdsIterable,
) -> Rc<dyn LinkQueryable> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iterable))
}

pub fn create_target_iterator_queryable(
    link_s2t_iterable: &impl SrcTgtIterable,
) -> Rc<dyn TargetIteratorQueryable> {
    Rc::new(TargetListMap::new(link_s2t_iterable))
}

pub fn create_source_iterator_queryable(
    link_s2t_iterable: &impl SrcTgtIterable,
) -> Rc<dyn SrcIterQueryable> {
    Rc::new(SrcListMap::new(link_s2t_iterable))
}
