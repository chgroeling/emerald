use super::vault;
use crate::types;

impl From<types::ResourceId> for vault::ResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<vault::ResourceId> for types::ResourceId {
    fn from(value: vault::ResourceId) -> Self {
        Self(value.0)
    }
}
