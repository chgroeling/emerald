#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

pub trait AllNoteLinksIterSource {
    type Iter: Iterator<Item = BacklinkRef>;
    fn all_iter(&self) -> Self::Iter;
}
