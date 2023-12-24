use crate::model::note;
use crate::model::vault;

impl From<&note::NoteMetadata> for vault::DocumentMetadata {
    fn from(value: &note::NoteMetadata) -> Self {
        Self {
            tags: value.document.tags.to_owned(),
            aliases: value.document.aliases.to_owned(),
            keywords: value.document.keywords.to_owned(),
            created: value.document.created.to_owned(),
            modified: value.document.modified.to_owned(),
        }
    }
}
