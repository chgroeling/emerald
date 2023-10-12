use crate::{md_analyzer::ContentType, types::Content};

pub fn extract_content_types<'a, I>(
    content: Content,
    md_analyzer: &'a I,
) -> impl Iterator<Item = ContentType> + 'static
where
    I: Fn(&String) -> Vec<ContentType>,
{
    md_analyzer(&content.0).into_iter()
}
