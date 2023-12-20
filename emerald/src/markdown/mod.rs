//! # Markdown Module
//!
//! This module provides functionality to parse and analyze Markdown text.
//! It includes utilities for iterating over Markdown elements, analyzing
//! content, and more.
//!

mod markdown_analyzer;
mod markdown_analyzer_impl;
mod markdown_analyzer_iter;
mod markdown_splitters;
mod states;
mod tests;
mod utf8_iterator;
mod utils;

pub use markdown_analyzer::MarkdownAnalyzer;
pub use markdown_analyzer_impl::MarkdownAnalyzerImpl;
pub use markdown_splitters::MarkdownFrontMatterSplitter;
