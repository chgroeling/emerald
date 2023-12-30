#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
pub struct VaultResourceId(pub Box<str>);

impl Eq for VaultResourceId {}
