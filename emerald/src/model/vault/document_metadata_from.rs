use crate::model::note::NoteMetadata;

use super::note::DocumentMetadata;

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
