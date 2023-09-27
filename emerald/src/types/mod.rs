mod content;
mod endpoint;
pub mod link;
mod link_2_tgt;
mod link_comps;
mod link_frm_src;
mod link_src_2_tgt;
mod res_and_err;
pub mod resource_id;
mod resource_id_comps;
pub mod split_link;
pub mod split_resoure_id;

pub use self::content::Content;
pub use self::res_and_err::EmeraldError;

pub type Result<T> = self::res_and_err::Result<T>;
pub use self::endpoint::EndPoint;
pub use self::link::Link;
pub use self::link_2_tgt::Link2Tgt;
pub use self::link_frm_src::LinkFrmSrc;
pub use self::link_src_2_tgt::LinkSrc2Tgt;
pub use self::resource_id::ResourceId;
