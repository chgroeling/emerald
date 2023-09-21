#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::OriginToDestination;

pub trait AllNoteLinksIterSource {
    type Iter: Iterator<Item = OriginToDestination>;
    fn all_iter(&self) -> Self::Iter;
}
