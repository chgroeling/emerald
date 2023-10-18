use crate::types::ContentType;

pub trait MarkdownAnalyzer<'a> {
    type Iterator: Iterator<Item = ContentType<'a>> + 'a;
    fn analyze(&self, md_str: &'a str) -> Self::Iterator;
}
