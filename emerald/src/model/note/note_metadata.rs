#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct FilesystemMetadata {
    pub path: String,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
}

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct DocumentMetadata {
    pub tags: Option<String>,
    pub aliases: Vec<String>,
    pub keywords: Vec<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct NoteMetadata {
    pub title: String,
    pub filesystem: FilesystemMetadata,
    pub document: DocumentMetadata,
}
