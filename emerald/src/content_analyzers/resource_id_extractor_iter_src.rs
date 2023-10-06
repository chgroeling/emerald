use crate::types::Link2Tgt;

pub trait ResourceIdExtractorIterSrc {
    type Iter: Iterator<Item = Link2Tgt>;
    fn iter(&self, content: String) -> Self::Iter;
}
