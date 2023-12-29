#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
pub struct ResourceId(pub Box<str>);

impl Eq for ResourceId {}
