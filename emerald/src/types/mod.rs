mod endpoint;
pub mod link;
pub mod link_decomposer;
mod res_and_err;
pub mod resource_id;

pub use self::res_and_err::EmeraldError;

pub type Link = self::link::Link;
pub type ResourceId = self::resource_id::ResourceId;
pub type Result<T> = self::res_and_err::Result<T>;
pub type LinkAndResourceId = (Link, Option<ResourceId>);
pub use self::endpoint::EndPoint;
