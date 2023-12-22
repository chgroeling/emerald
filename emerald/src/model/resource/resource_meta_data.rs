use crate::types;

#[derive(Clone)]
pub struct ResourceMetaData {
    pub name: String,
    pub resource_type: types::ResourceType,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
}

impl From<types::FilesystemMetaData> for ResourceMetaData {
    fn from(value: types::FilesystemMetaData) -> Self {
        Self {
            name: value.name,
            resource_type: value.resource_type,
            size: value.size,
            modified: value.modified,
            created: value.created,
        }
    }
}
