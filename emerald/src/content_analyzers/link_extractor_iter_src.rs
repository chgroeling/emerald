use crate::types::{Content, Link};

pub trait LinkExtractorIterSrc {
    type Iter: Iterator<Item = Link>;
    fn iter(&self, content: Content) -> Self::Iter;
}
