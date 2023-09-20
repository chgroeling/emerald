#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::NoteLink;

pub trait AllNoteLinksIterSource {
    type Iter: Iterator<Item = NoteLink>;
    fn all_iter(&self) -> Self::Iter;
}
