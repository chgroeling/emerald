use crate::types::LinkSrc2Tgt;

mod extract_links_from_vault;
mod trafo_to_content;
mod trafo_to_content_types;
mod trafo_to_filetype_and_resource_id;
mod trafo_to_link_2_tgt;
mod trafo_to_link_src_2_tgt;
mod trafo_to_links;
mod trafo_to_resource_id;

pub use extract_links_from_vault::trafo_links_from_contents;
pub use trafo_to_content::trafo_resource_ids_to_content;
pub use trafo_to_filetype_and_resource_id::trafo_to_filetype_and_resource_id;
pub use trafo_to_resource_id::filter_markdown_types;
pub use trafo_to_resource_id::trafo_ep_to_rid;

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;
