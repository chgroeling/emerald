#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::LinkOriginDestination;

pub trait AllNoteLinksIterSource {
    type Iter: Iterator<Item = LinkOriginDestination>;
    fn all_iter(&self) -> Self::Iter;
}
