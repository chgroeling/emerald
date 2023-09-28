use super::resource_id_comps::ResourceIdComps;

#[derive(Debug, Clone, PartialEq, Hash)]

/// A ResourceId points to a unique Resource
///
/// Currently a ResourceId is nothing else than a string containing a path
/// to the filesystem
pub struct ResourceId(pub String);

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

impl From<&ResourceIdComps> for ResourceId {
    fn from(value: &ResourceIdComps) -> Self {
        Self(value.to_string())
    }
}
