pub trait ResourceIdTrait: std::fmt::Debug {}

// Blanket impl
impl<T> ResourceIdTrait for T where T: std::fmt::Debug {}
