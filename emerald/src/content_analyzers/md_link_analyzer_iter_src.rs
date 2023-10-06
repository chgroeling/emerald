use crate::types::{Content, Link2Tgt};

pub trait MdLinkAnalyzerIterSrc {
    type Iter: Iterator<Item = Link2Tgt>;
    fn iter(&self, content: Content) -> Self::Iter;
}
