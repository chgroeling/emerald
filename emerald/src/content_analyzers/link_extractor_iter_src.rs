use crate::types::Link;

pub trait LinkExtractorIterSrc {
    type Iter: Iterator<Item = Link>;
    fn iter(&self, content: String) -> Self::Iter;
}
