use crate::types::LinkSrc2Tgt;

mod extract_links_from_vault;
mod trafo_to_content_types;
mod trafo_to_filetype_and_resource_id;
mod trafo_to_link_2_tgt;
mod trafo_to_link_src_2_tgt;
mod trafo_to_links;

pub use extract_links_from_vault::extract_links_from_vault;
pub use trafo_to_filetype_and_resource_id::trafo_to_filetype_and_resource_id;

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;
