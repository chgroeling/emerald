#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::EndPoint;

pub trait AllEndpointsIterSource {
    type Iter: Iterator<Item = EndPoint>;
    fn all_iter(&self) -> Self::Iter;
}
