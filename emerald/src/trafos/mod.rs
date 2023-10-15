use crate::types::LinkSrc2Tgt;

mod trafo_to_content;
mod trafo_to_content_types;
mod trafo_to_filetype_and_resource_id;
mod trafo_to_link_2_tgt;
mod trafo_to_link_src_2_tgt;
mod trafo_to_links;
mod trafo_to_linksrc2tgt;
mod trafo_to_name;
mod trafo_to_resource_id;

pub use trafo_to_content::trafo_from_res_ids_to_content;
pub use trafo_to_filetype_and_resource_id::trafo_to_filetype_and_res_id;
pub use trafo_to_linksrc2tgt::trafo_from_content_to_linksrc2tgt;
pub use trafo_to_name::trafo_from_res_id_to_name;
pub use trafo_to_resource_id::filter_markdown_types;
pub use trafo_to_resource_id::trafo_ep_to_rid;

pub type LinkSrc2TgtIterBoxed<'a> = Box<dyn Iterator<Item = LinkSrc2Tgt> + 'a>;
