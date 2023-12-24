use super::note::{DocumentMetadata, FilesystemMetadata};
use super::note_factory::NoteFactory;
use super::Note;
use crate::markdown::MarkdownFrontMatterSplitter;
use crate::model::{content, note};
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    metadata_retriever: Rc<dyn note::NoteMetadataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetadataRetriever>,
        content_retriever: Rc<dyn content::MdContentRetriever>,
    ) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
            content_retriever,
        }
    }
}

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        let note_md = self.metadata_retriever.retrieve(&rid);

        let filesystem_md: FilesystemMetadata = note_md.into();
        let document_md: DocumentMetadata = note_md.into();
        let content = self.content_retriever.retrieve(&rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

        Note::new(
            rid.into(),
            note_md.title.clone(),
            yaml_str.to_string(),
            markdown.to_string(),
            filesystem_md,
            document_md,
        )
    }
}
