#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::LinkFromSourceToTarget;

pub trait LinkFromSourceToTargetIterable {
    type Iter: Iterator<Item = LinkFromSourceToTarget>;
    fn iter(&self) -> Self::Iter;
}
