use crate::{md_analyzer::ContentType, types::Content};

pub fn trafo_from_content_to_content_type<'a, I>(
    content: Content,
    md_analyzer: &'a I,
) -> impl Iterator<Item = ContentType> + 'static
where
    I: Fn(&String) -> Vec<ContentType>,
{
    md_analyzer(&content.0).into_iter()
}
