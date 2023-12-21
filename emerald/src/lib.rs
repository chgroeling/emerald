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
mod yaml;

pub use crate::emerald::Emerald;
pub use crate::error::{EmeraldError, Result};
pub use crate::vault::Note;
pub use crate::vault::NoteTypes;
pub use crate::vault::Vault;
