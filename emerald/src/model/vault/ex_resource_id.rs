#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
pub struct ExResourceId(pub Box<str>);

impl Eq for ExResourceId {}

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct VaultResourceId<T: std::fmt::Debug + std::hash::Hash + Eq>(pub T);

impl<T: std::fmt::Debug + std::hash::Hash + Eq> Eq for VaultResourceId<T> {}
