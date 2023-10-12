use crate::{md_analyzer::markdown_analyzer::MarkdownExtractorIter, types::Content};

use super::content_type::ContentType;

pub fn extract_content_types<'a>(content: Content) -> impl Iterator<Item = ContentType> + 'a {
    MarkdownExtractorIter::new(content)
}
