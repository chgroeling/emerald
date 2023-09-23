use std::rc::Rc;

mod link_queryable;
mod resource_id_link_map;
mod source_iterator_queryable;
mod source_list_map;
mod target_iterator_queryable;
mod target_list_map;

pub use self::link_queryable::LinkQueryable;
pub use self::source_iterator_queryable::SourceIteratorQueryable;
pub use self::target_iterator_queryable::TargetIteratorQueryable;

use self::{
    resource_id_link_map::ResourceIdLinkMap, source_list_map::SourceListMap,
    target_list_map::TargetListMap,
};
use crate::indexes::{LinkFromSourceToTargetIterable, ResourceIdsIterable};

pub fn create_link_queryable(
    resource_ids_iterable: &impl ResourceIdsIterable,
) -> Rc<dyn LinkQueryable> {
    Rc::new(ResourceIdLinkMap::new(resource_ids_iterable))
}

pub fn create_target_iterator_queryable(
    link_s2t_iterable: &impl LinkFromSourceToTargetIterable,
) -> Rc<dyn TargetIteratorQueryable> {
    Rc::new(TargetListMap::new(link_s2t_iterable))
}

pub fn create_source_iterator_queryable(
    link_s2t_iterable: &impl LinkFromSourceToTargetIterable,
) -> Rc<dyn SourceIteratorQueryable> {
    Rc::new(SourceListMap::new(link_s2t_iterable))
}
