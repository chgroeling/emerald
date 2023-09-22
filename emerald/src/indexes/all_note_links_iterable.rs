#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::LinkFromSourceToTarget;

pub trait AllNoteLinksIterable {
    type Iter: Iterator<Item = LinkFromSourceToTarget>;
    fn all_iter(&self) -> Self::Iter;
}
