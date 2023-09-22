use crate::types::LinkToTarget;

pub trait MdLinkAnalyzerIterable {
    type Iter: Iterator<Item = LinkToTarget>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
