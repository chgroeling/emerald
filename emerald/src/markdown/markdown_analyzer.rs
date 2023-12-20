/// A trait for analyzing Markdown text.
///
/// `MarkdownAnalyzer` provides functionality to analyze Markdown text
/// and iterate over its constituent blocks (like paragraphs, headers, lists, etc.).
/// It leverages an associated type, `Iterator`, which is an iterator over
/// `MdBlock` items.
///
/// # Type Parameters
///
/// * `'a` - Lifetime parameter indicating the lifetime of the markdown string slice.
///
/// # Associated Types
///
/// * `Iterator`: The type of iterator returned by the `analyze` method. It must implement
/// the `Iterator` trait and yield items of type `types::MdBlock<'a>`.
///
use crate::types;

pub trait MarkdownAnalyzer<'a> {
    type Iterator: Iterator<Item = types::MdBlock<'a>> + 'a;

    /// Analyzes the provided markdown string and returns an iterator over the markdown blocks.
    ///
    /// This method takes a markdown string slice and returns an iterator
    /// of type `Self::Iterator`, which yields `MdBlock` items representing
    /// the different blocks of content within the markdown text.
    fn analyze(&self, md_str: &'a str) -> Self::Iterator;
}
