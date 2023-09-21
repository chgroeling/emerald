mod content;
mod endpoint;
pub mod link;
mod link_components;
pub mod link_decomposer;
mod link_origin_destination;
mod res_and_err;
pub mod resource_id;

pub use self::content::Content;
pub use self::res_and_err::EmeraldError;

pub type Link = self::link::Link;
pub type ResourceId = self::resource_id::ResourceId;
pub type Result<T> = self::res_and_err::Result<T>;
pub type LinkAndResourceId = (Link, Option<ResourceId>);
pub use self::endpoint::EndPoint;
pub use self::link_origin_destination::LinkOriginDestination;
