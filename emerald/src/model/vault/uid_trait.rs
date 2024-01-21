pub trait UidTrait: std::fmt::Debug + std::hash::Hash + Eq + Clone {}

// Blanket impl
impl<T> UidTrait for T where T: std::fmt::Debug + std::hash::Hash + Eq + Clone {}
