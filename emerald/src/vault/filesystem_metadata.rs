use super::timestamp::Timestamp;

pub struct FilesystemMetadata {
    pub location: String,
    pub size: u64,
    pub modified: Timestamp,
    pub created: Timestamp,
}
