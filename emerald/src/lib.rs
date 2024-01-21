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

pub use crate::emerald::DefaultEmerald;
pub use crate::emerald::Emerald;
pub use crate::error::EmeraldError;
pub use crate::error::Result;
pub use crate::model::unique_id::Uid;
use crate::model::vault::Note;
pub use crate::model::vault::NoteTypes;
pub use crate::types::ResourceId;

pub type EmeraldNote = Note<Uid>;
