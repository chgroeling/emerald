use std::rc::Rc;

mod destination_list_cache;
mod destination_list_resolver;
mod link_resolver;
mod resource_id_link_resolver;

pub use self::link_resolver::LinkResolver;
use self::resource_id_link_resolver::ResourceIdLinkResolver;
use crate::indexes::AllResourceIdsIterSource;

pub fn create_link_resolver(ep_iter_src: &impl AllResourceIdsIterSource) -> Rc<dyn LinkResolver> {
    Rc::new(ResourceIdLinkResolver::new(ep_iter_src))
}
