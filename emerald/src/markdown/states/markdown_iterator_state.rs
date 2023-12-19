use crate::markdown::utf8_iterator::Utf8Iterator;

#[derive(PartialEq, Debug, Clone)]
pub enum State {
    // This state is assigned when parsing text.
    Text,

    /// This state is assigned at the start of parsing
    DocumentStart,

    /// Empty line was found
    EmptyLine,

    /// New line character was found
    NewLine,

    /// Yaml Frontmatter was found
    YamlFrontmatter,

    /// Inline Code Block was found
    InlCodeBlock,
}

pub enum Yield {
    YamlFrontmatter(usize, usize),
    CodeBlock(usize, usize),
    WikiLink(usize, usize),
    Link(usize, usize),
}

pub enum ActionResult {
    EndOfFile,
    NextState(State),
    YieldState(State, Yield),
}

#[derive(Debug)]
pub struct StateData<'a> {
    pub state: State,
    pub it: Utf8Iterator<'a>,
}
