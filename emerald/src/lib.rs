mod adapters;
mod emerald;
mod error;
mod markdown;
mod model;
mod resources;
mod stats;
mod types;
mod utils;
mod yaml;

pub use crate::emerald::Emerald;
pub use crate::error::{EmeraldError, Result};
pub use crate::model::vault::Note;
pub use crate::model::vault::NoteTypes;
pub use crate::model::vault::Vault;
