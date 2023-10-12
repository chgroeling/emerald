pub enum MarkdownIteratorState {
    IllegalFormat,

    // Inline Code Block Start
    InlCodeBlockStart(usize),

    // Inline Code Block Found
    InlCodeBlockFound(usize, usize),

    CodeBlockStart(usize),
    CodeBlockFound(usize, usize),

    WikiLinkStart(usize),
    WikiLinkFound(usize, usize),

    LinkStart(usize),
    LinkDescriptionFinished(usize),
    LinkFound(usize, usize),
}
