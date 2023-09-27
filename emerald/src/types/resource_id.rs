use super::link::Link;

#[derive(Debug, Clone, PartialEq, Hash)]

/// A ResourceId points to a unique Resource
///
/// Currently a ResourceId is nothing else than a string containing a path
/// to the filesystem
pub struct ResourceId(pub String);

impl ResourceId {
    #[allow(dead_code)]
    pub fn downgrad(self) -> Link {
        Link(self.0)
    }
}

// Allows to use a string as a ResourceId
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
