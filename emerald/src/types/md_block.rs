#[derive(PartialEq, Debug)]
pub enum MdBlock<'a> {
    WikiLink(&'a str),
    Link(&'a str),
    CodeBlock(&'a str),
}
