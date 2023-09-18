use crate::types::LinkAndResourceId;

pub trait MdLinkAnalyzerIterSource {
    type Iter: Iterator<Item = LinkAndResourceId>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
