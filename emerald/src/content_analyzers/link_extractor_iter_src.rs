use crate::types::Link;

pub trait LinkExtractorIterSrc {
    type Iter: Iterator<Item = Link>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
