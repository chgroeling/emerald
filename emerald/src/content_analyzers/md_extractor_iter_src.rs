use crate::types::Content;

use super::content_type::ContentType;

pub trait MarkdownExtractorIterSrc {
    type Iter: Iterator<Item = ContentType>;
    fn iter(&self, content: Content) -> Self::Iter;
}
