use std::rc::Rc;

mod link_queryable;
mod resource_id_link_map;
mod target_iterator_queryable;
mod target_list_map;

pub use self::link_queryable::LinkQueryable;
pub use self::target_iterator_queryable::TargetIteratorQueryable;

use self::{resource_id_link_map::ResourceIdLinkMap, target_list_map::TargetListMap};
use crate::indexes::{AllNoteLinksIterable, AllResourceIdsIterSource};

pub fn create_link_queryable(
    all_resource_ids_iter_source: &impl AllResourceIdsIterSource,
) -> Rc<dyn LinkQueryable> {
    Rc::new(ResourceIdLinkMap::new(all_resource_ids_iter_source))
}

pub fn create_target_iterator_queryable(
    all_note_links_iter_source: &impl AllNoteLinksIterable,
) -> Rc<dyn TargetIteratorQueryable> {
    Rc::new(TargetListMap::new(all_note_links_iter_source))
}
