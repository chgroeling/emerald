use crate::types::Link;

pub trait LinkExtractorIterSource {
    type Iter: Iterator<Item = Link>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
