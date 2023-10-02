use crate::types::Link2Tgt;

pub trait ResourceIdExtractorIterSource {
    type Iter: Iterator<Item = Link2Tgt>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
