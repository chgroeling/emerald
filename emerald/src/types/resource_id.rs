use super::link::Link;

#[derive(Debug, Clone, PartialEq, Hash)]
/// Endpoint Link
pub struct ResourceId(pub String);

impl ResourceId {
    #[allow(dead_code)]
    pub fn downgrad(self) -> Link {
        Link(self.0)
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Eq for ResourceId {}
