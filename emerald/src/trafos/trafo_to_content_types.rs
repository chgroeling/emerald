use crate::types::Content;
use crate::types::ContentType;

pub fn trafo_from_content_to_content_type<'a, I, Iter>(
    content: &'a Content,
    md_analyzer: &'a I,
) -> impl Iterator<Item = ContentType> + 'a
where
    I: Fn(&'a String) -> Iter,
    Iter: Iterator<Item = ContentType> + 'a,
{
    md_analyzer(&content.0)
}
