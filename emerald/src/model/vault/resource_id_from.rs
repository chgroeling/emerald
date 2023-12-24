use crate::types;

use super::ResourceId;

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
