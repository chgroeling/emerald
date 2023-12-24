use crate::types;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct FilesystemMetadata {
    pub location: String,
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

#[derive(Clone)]
pub struct NoteMetadata {
    pub title: String,
    pub filesystem: FilesystemMetadata,
    pub document: DocumentMetadata,
}

impl From<(types::FilesystemMetadata, types::DocumentMetadata)> for NoteMetadata {
    fn from(value: (types::FilesystemMetadata, types::DocumentMetadata)) -> Self {
        let fs_md = FilesystemMetadata {
            location: value.0.location,
            size: value.0.size,
            modified: value.0.modified,
            created: value.0.created,
        };

        let doc_md = DocumentMetadata {
            aliases: value.1.aliases.unwrap_or(vec![]),
            keywords: value.1.keywords.unwrap_or(vec![]),
            modified: value.1.modified,
            created: value.1.created,
            tags: value.1.tags,
        };

        Self {
            title: value.0.name,
            filesystem: fs_md,
            document: doc_md,
        }
    }
}
