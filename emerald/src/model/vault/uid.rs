#[derive(Debug, Clone, PartialEq, Hash, Default)]

pub struct Uid(pub Box<str>);

impl Eq for Uid {}
