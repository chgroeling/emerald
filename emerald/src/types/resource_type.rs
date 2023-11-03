#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ResourceType {
    Unknown(String),
    Markdown(String),
    NoType(), // No resource type available
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::NoType()
    }
}
