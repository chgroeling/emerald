use crate::types::{Content, Link2Tgt};

pub trait ResourceIdExtractorIterSrc {
    type Iter: Iterator<Item = Link2Tgt>;
    fn iter(&self, content: Content) -> Self::Iter;
}
