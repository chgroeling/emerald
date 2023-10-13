#[derive(PartialEq, Debug)]
pub enum ContentType<'a> {
    WikiLink(&'a str),
    Link(&'a str),
    CodeBlock(&'a str),
}
