use crate::{
    md_analyzer::{analyze_markdown, ContentType},
    types::Content,
};

pub fn extract_content_types(content: Content) -> impl Iterator<Item = ContentType> {
    analyze_markdown(&content.0).into_iter()
}
