use crate::types::LinkAndDestination;

pub trait MdLinkAnalyzerIterSource {
    type Iter: Iterator<Item = LinkAndDestination>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
