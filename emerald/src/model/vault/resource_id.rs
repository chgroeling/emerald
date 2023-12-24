use crate::types;

#[derive(Debug, Clone, PartialEq, Hash, Default)]

/// A ResourceId points to a unique Resource
///
/// Currently a ResourceId is nothing else than a string containing a path
/// to the filesystem
pub struct ResourceId(pub String);

impl From<types::ResourceId> for ResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<ResourceId> for types::ResourceId {
    fn from(value: ResourceId) -> Self {
        Self(value.0)
    }
}
