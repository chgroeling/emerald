use crate::model::vault;
use crate::types;

impl From<types::ResourceId> for vault::ResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0.into_string())
    }
}

impl From<vault::ResourceId> for types::ResourceId {
    fn from(value: vault::ResourceId) -> Self {
        Self(value.0.into_boxed_str())
    }
}
