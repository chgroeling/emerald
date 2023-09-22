mod content;
mod endpoint;
pub mod link;
mod link_components;
pub mod link_decomposer;
mod link_from_source;
mod link_from_source_to_target;
mod link_to_target;
mod res_and_err;
pub mod resource_id;

pub use self::content::Content;
pub use self::res_and_err::EmeraldError;

pub type Link = self::link::Link;
pub type ResourceId = self::resource_id::ResourceId;
pub type Result<T> = self::res_and_err::Result<T>;
pub type LinkToTarget = self::link_to_target::LinkToTarget;
pub type LinkFromSource = self::link_from_source::LinkFromSource;
pub use self::endpoint::EndPoint;
pub use self::link_from_source_to_target::LinkFromSourceToTarget;
