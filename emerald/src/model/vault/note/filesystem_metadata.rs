use super::timestamp::Timestamp;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct FilesystemMetadata {
    pub path: String,
    pub size: u64,
    pub modified: Timestamp,
    pub created: Timestamp,
}
