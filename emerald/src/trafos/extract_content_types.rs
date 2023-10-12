use crate::{md_analyzer::markdown_analyzer::analyze_markdown, types::Content};

use super::content_type::ContentType;

pub fn extract_content_types(content: Content) -> impl Iterator<Item = ContentType> {
    analyze_markdown(&content.0).into_iter()
}
