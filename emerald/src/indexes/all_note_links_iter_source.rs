#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::SourceAndLinkToTarget;

pub trait AllNoteLinksIterSource {
    type Iter: Iterator<Item = SourceAndLinkToTarget>;
    fn all_iter(&self) -> Self::Iter;
}
