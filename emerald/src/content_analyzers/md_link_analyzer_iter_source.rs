use crate::types::LinkToTarget;

pub trait MdLinkAnalyzerIterSource {
    type Iter: Iterator<Item = LinkToTarget>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
