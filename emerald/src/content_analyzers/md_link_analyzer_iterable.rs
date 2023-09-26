use crate::types::Link2Tgt;

pub trait MdLinkAnalyzerIterable {
    type Iter: Iterator<Item = Link2Tgt>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
