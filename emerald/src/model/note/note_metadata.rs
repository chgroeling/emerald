use crate::types;

#[derive(Clone)]
pub struct NoteMetadata {
    pub title: String,
    pub location: String,
    pub size: u64,
    pub modified: i64,
    pub created: i64,
    pub aliases: Vec<String>,
}

impl From<(types::FilesystemMetadata, types::DocumentMetadata)> for NoteMetadata {
    fn from(value: (types::FilesystemMetadata, types::DocumentMetadata)) -> Self {
        let aliases = if let Some(a) = value.1.aliases {
            a
        } else {
            vec![]
        };
        Self {
            title: value.0.name,
            location: value.0.location,
            size: value.0.size,
            modified: value.0.modified,
            created: value.0.created,
            aliases,
        }
    }
}
