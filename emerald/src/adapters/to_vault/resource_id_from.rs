use crate::model::vault;
use crate::types;

impl From<types::ResourceId> for vault::VaultResourceId {
    fn from(value: types::ResourceId) -> Self {
        Self(value.0)
    }
}

impl From<vault::VaultResourceId> for types::ResourceId {
    fn from(value: vault::VaultResourceId) -> Self {
        Self(value.0)
    }
}
