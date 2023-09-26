mod content;
mod endpoint;
pub mod link;
mod link_2_tgt;
mod link_components;
pub mod link_decomposer;
mod link_frm_src;
mod link_src_2_tgt;
mod res_and_err;
pub mod resource_id;

pub use self::content::Content;
pub use self::res_and_err::EmeraldError;

pub type Link = self::link::Link;
pub type ResourceId = self::resource_id::ResourceId;
pub type Result<T> = self::res_and_err::Result<T>;
pub type LinkToTarget = self::link_2_tgt::Link2Tgt;
pub type LinkFromSource = self::link_frm_src::LinkFrmSrc;
pub use self::endpoint::EndPoint;
pub use self::link_src_2_tgt::LinkSrc2Tgt;
