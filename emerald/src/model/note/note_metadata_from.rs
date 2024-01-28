use super::note_metadata::{DocumentMetadata, FilesystemMetadata, NoteMetadata};
use crate::types;

impl From<(types::FilesystemMetadata, types::DocumentMetadata)> for NoteMetadata {
    fn from(value: (types::FilesystemMetadata, types::DocumentMetadata)) -> Self {
        let path = value
            .0
            .path
            .to_str()
            .expect("Path must have a valid utf-8 representation");

        let fs_md = FilesystemMetadata {
            path: path.to_owned(),
            size: value.0.size,
            modified: value.0.modified,
            created: value.0.created,
        };

        let doc_md = DocumentMetadata {
            uid: value.1.uid,
            aliases: value.1.aliases.unwrap_or_default(),
            keywords: value.1.keywords.unwrap_or_default(),
            modified: value.1.modified,
            created: value.1.created,
            tags: value.1.tags,
        };

        // get name of file
        let os_filename = value.0.path.file_stem().expect("Filename expected");
        // Currently the title is nothing else than the filename without extension
        let title = os_filename
            .to_str()
            .expect("Filename must have a valid utf-8 representation")
            .to_owned();

        Self {
            title,
            filesystem: fs_md,
            document: doc_md,
        }
    }
}
