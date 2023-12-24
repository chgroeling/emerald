use super::note_factory::NoteFactory;
use super::timestamp::Timestamp;
use super::Note;
use crate::markdown::MarkdownFrontMatterSplitter;
use crate::model::{content, note};
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    meta_data_retriever: Rc<dyn note::NoteMetadataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetadataRetriever>,
        content_retriever: Rc<dyn content::MdContentRetriever>,
    ) -> Self {
        Self {
            meta_data_retriever,
            content_retriever,
        }
    }
}

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, rid: types::ResourceId) -> Note {
        let metadata = self.meta_data_retriever.retrieve(&rid);
        let content = self.content_retriever.retrieve(&rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

        Note::new(
            rid,
            metadata.title.clone(),
            metadata.document.aliases.clone(),
            yaml_str.to_string(),
            metadata.filesystem.location.clone(),
            markdown.to_string(),
            metadata.filesystem.size,
            Timestamp(metadata.filesystem.created),
            Timestamp(metadata.filesystem.modified),
        )
    }
}
