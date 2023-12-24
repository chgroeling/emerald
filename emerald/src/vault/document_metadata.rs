use crate::model::note::NoteMetadata;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub struct DocumentMetadata {
    pub tags: Option<String>,
    pub aliases: Vec<String>,
    pub keywords: Vec<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl From<&NoteMetadata> for DocumentMetadata {
    fn from(value: &NoteMetadata) -> Self {
        Self {
            tags: value.document.tags.to_owned(),
            aliases: value.document.aliases.to_owned(),
            keywords: value.document.keywords.to_owned(),
            created: value.document.created.to_owned(),
            modified: value.document.modified.to_owned(),
            ..Default::default()
        }
    }
}
