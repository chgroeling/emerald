#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
pub struct ExResourceId(pub Box<str>);

impl Eq for ExResourceId {}

pub trait VaultResourceIdTrait: std::fmt::Debug + std::hash::Hash + Eq + Clone {}

// Blanket impl
impl<T> VaultResourceIdTrait for T where T: std::fmt::Debug + std::hash::Hash + Eq + Clone {}

#[derive(Debug, PartialEq, Clone, Hash, Default)]
pub struct VaultResourceId<T: VaultResourceIdTrait>(pub T);

impl<T: VaultResourceIdTrait> Eq for VaultResourceId<T> {}
