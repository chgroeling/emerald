pub trait VaultResourceIdTrait: std::fmt::Debug + std::hash::Hash + Eq + Clone {}

// Blanket impl
impl<T> VaultResourceIdTrait for T where T: std::fmt::Debug + std::hash::Hash + Eq + Clone {}
