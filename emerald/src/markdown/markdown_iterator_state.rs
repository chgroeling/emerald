#[derive(PartialEq, Debug, Clone)]
pub enum MarkdownIteratorState {
    IllegalFormat,
    StartOfParsing, // State at the start of parsing
    EmptyLineFound, // State that an empty line was found
    NewLineFound,

    YamlFrontmatterFound(usize, usize),

    // Inline Code Block Start
    InlCodeBlockStart(usize),

    // Inline Code Block Found
    InlCodeBlockFound(usize, usize),

    CodeBlockFound(usize, usize),

    WikiLinkStart(usize),
    WikiLinkFound(usize, usize),

    LinkStart(usize),
    LinkDescriptionFinished(usize),
    LinkFound(usize, usize),
}
