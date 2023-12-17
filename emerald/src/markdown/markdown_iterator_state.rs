use std::{iter::Peekable, str::CharIndices};

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    // This state is assigned when parsing text.
    TextState,

    /// This state is assigned at the start of parsing
    DocumentStartState,

    /// Empty line was found
    EmptyLineState,

    /// New line character was found
    NewLineState,

    /// Yaml Frontmatter was found
    YamlFrontmatterState,

    /// Inline Code Block was found
    InlCodeBlockState,
}

pub enum YieldResult {
    YamlFrontmatter(usize, usize),
    CodeBlock(usize, usize),
    WikiLink(usize, usize),
    Link(usize, usize),
}

pub enum ActionResult {
    NextState(State),
    YieldState(State, YieldResult),
}

#[derive(Debug)]
pub struct StateData<'a> {
    pub state: State,
    pub it: Peekable<CharIndices<'a>>,
}
