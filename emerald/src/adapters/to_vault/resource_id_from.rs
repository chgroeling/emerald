use crate::model::vault;
use crate::types;

impl From<types::ResourceId> for vault::ExResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<vault::ExResourceId> for types::ResourceId {
    fn from(value: vault::ExResourceId) -> Self {
        Self(value.0)
    }
}
