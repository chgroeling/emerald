use std::path::PathBuf;

use crate::types;

#[derive(Clone)]
pub struct ResourceMetadata {
    pub path: PathBuf,
    pub resource_type: types::ResourceType,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
}

impl From<&types::FilesystemMetadata> for ResourceMetadata {
    fn from(value: &types::FilesystemMetadata) -> Self {
        Self {
            path: value.path.clone(),
            resource_type: value.resource_type.clone(),
            size: value.size,
            modified: value.modified,
            created: value.created,
        }
    }
}
