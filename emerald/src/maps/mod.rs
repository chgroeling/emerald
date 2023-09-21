use std::rc::Rc;

mod destination_list_map;
mod destination_list_resolver;
mod link_queryable;
mod resource_id_link_map;

pub use self::destination_list_resolver::DestinationListResolver;
pub use self::link_queryable::LinkQueryable;

use self::{
    destination_list_map::DestinationListMap, resource_id_link_map::ResourceIdLinkResolver,
};
use crate::indexes::{AllNoteLinksIterSource, AllResourceIdsIterSource};

pub fn create_link_queryable(
    all_resource_ids_iter_source: &impl AllResourceIdsIterSource,
) -> Rc<dyn LinkQueryable> {
    Rc::new(ResourceIdLinkResolver::new(all_resource_ids_iter_source))
}

pub fn create_destination_list_resolver(
    all_note_links_iter_source: &impl AllNoteLinksIterSource,
) -> Rc<dyn DestinationListResolver> {
    Rc::new(DestinationListMap::new(all_note_links_iter_source))
}
