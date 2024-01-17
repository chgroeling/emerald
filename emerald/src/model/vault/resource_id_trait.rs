pub trait ResourceIdTrait: std::fmt::Debug + std::hash::Hash + Eq + Clone {}

// Blanket impl
impl<T> ResourceIdTrait for T where T: std::fmt::Debug + std::hash::Hash + Eq + Clone {}
