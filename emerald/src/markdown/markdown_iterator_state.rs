use std::{iter::Peekable, str::CharIndices};

#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownIteratorState {
    // This state is assigned when parsing text.
    Text,

    /// This state is assigned at the start of parsing
    DocumentStart,

    /// Empty line was found
    EmptyLineFound,

    /// New line character was found
    NewLineFound,

    /// Yaml Frontmatter was found
    YamlFrontmatterFound,

    /// Inline Code Block was found
    InlCodeBlockFound,

    LinkStart(usize),
    LinkDescriptionFinished(usize),
}

pub enum YieldResut {
    /// Yaml Frontmatter was found
    YamlFrontmatter(usize, usize),

    CodeBlock(usize, usize),

    WikiLink(usize, usize),

    Link(usize, usize),
}

pub enum ActionResult {
    NextState(MarkdownIteratorState),
    YieldState(MarkdownIteratorState, YieldResut),
}

#[derive(Debug)]
pub struct StateData<'a> {
    pub state: MarkdownIteratorState,
    pub it: Peekable<CharIndices<'a>>,
}
