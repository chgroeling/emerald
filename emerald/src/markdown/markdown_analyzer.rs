use crate::types;

pub trait MarkdownAnalyzer<'a> {
    type Iterator: Iterator<Item = types::MdBlock<'a>> + 'a;
    fn analyze(&self, md_str: &'a str) -> Self::Iterator;
}
