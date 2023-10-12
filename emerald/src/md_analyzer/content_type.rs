#[derive(PartialEq, Debug)]
pub enum ContentType {
    WikiLink(String),
    Link(String),
    CodeBlock(String),
}
