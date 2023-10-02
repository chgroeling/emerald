use super::content_type::ContentType;

pub trait MarkdownExtractorIterSource {
    type Iter: Iterator<Item = ContentType>;
    fn create_iter(&self, content: String) -> Self::Iter;
}
