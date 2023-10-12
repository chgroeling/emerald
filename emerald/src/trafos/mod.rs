use crate::types::LinkSrc2Tgt;

mod convert_to_link_2_tgt;
mod convert_to_link_src_2_tgt;
mod extract_content_types;
mod extract_links;
mod extract_links_from_content;
mod trafo_to_filetype_and_resource_id;

pub use extract_links_from_content::extract_links_from_vault;
pub use trafo_to_filetype_and_resource_id::trafo_to_filetype_and_resource_id;

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;
