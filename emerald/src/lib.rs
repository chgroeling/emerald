mod content_analyzers;
mod indexes;
mod maps;
mod resources;
mod types;
mod utils;

pub mod emerald;

pub type Emerald = crate::emerald::Emerald;

pub use crate::types::EmeraldError;
pub type Result<T> = crate::types::Result<T>;
