use crate::types::Content;

use super::{content_type::ContentType, markdown_analyzer::MarkdownExtractorIter};

pub fn extract_content_types<'a>(content: Content) -> impl Iterator<Item = ContentType> + 'a {
    MarkdownExtractorIter::new(content)
}
