use std::rc::Rc;

mod destination_list_map;
mod destination_list_resolver;
mod link_resolver;
mod resource_id_link_map;

pub use self::destination_list_resolver::DestinationListResolver;
pub use self::link_resolver::LinkResolver;

use self::{
    destination_list_map::DestinationListMap, resource_id_link_map::ResourceIdLinkResolver,
};
use crate::indexes::{AllNoteLinksIterSource, AllResourceIdsIterSource};

pub fn create_link_resolver(ep_iter_src: &impl AllResourceIdsIterSource) -> Rc<dyn LinkResolver> {
    Rc::new(ResourceIdLinkResolver::new(ep_iter_src))
}

pub fn create_destination_list_resolver(
    all_note_links_iter_source: &impl AllNoteLinksIterSource,
) -> Rc<dyn DestinationListResolver> {
    Rc::new(DestinationListMap::new(all_note_links_iter_source))
}
