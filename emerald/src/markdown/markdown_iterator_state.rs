#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownIteratorState {
    IllegalFormat,

    /// This state is assigned at the start of parsing
    StartOfParsing,

    /// Empty line was found
    EmptyLineFound,

    /// New line character was found
    NewLineFound,

    /// Yaml Frontmatter was found
    YamlFrontmatterFound(usize, usize),

    /// Inline Code Block was found
    InlCodeBlockFound(usize, usize),

    CodeBlockFound(usize, usize),

    WikiLinkFound(usize, usize),

    LinkStart(usize),
    LinkDescriptionFinished(usize),
    LinkFound(usize, usize),
}

pub enum ActionResult {
    // Stay,
    NextState(MarkdownIteratorState),
    Yield(MarkdownIteratorState),
}
