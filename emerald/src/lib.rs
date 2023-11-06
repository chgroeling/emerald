mod adapters;
mod emerald;
mod error;
mod markdown;
mod model;
mod resources;
mod stats;
mod types;
mod utils;
mod vault;

pub use crate::emerald::Emerald;
pub use crate::error::{EmeraldError, Result};
pub use crate::vault::Note;
pub use crate::vault::Vault;
