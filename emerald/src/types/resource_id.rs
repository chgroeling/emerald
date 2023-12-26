#[derive(Debug, Clone, PartialEq, Hash)]

/// A ResourceId points to a unique Resource
///
/// Currently a ResourceId is nothing else than a string containing a path
/// to the filesystem
pub struct ResourceId(pub Box<str>);

// Allows to use a string as a ResourceId
impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned().into_boxed_str())
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value.into_boxed_str())
    }
}

impl Eq for ResourceId {}
