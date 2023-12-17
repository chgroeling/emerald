use super::note_factory::NoteFactory;
use super::timestamp::Timestamp;
use super::Note;
use crate::markdown::MarkdownFrontMatterSplitter;
use crate::model::{content, note};
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
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
        let meta_data = self.meta_data_retriever.retrieve(&rid);
        let content = self.content_retriever.retrieve(&rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

        Note::new(
            rid,
            meta_data.title.clone(),
            yaml_str,
            meta_data.location.clone(),
            markdown,
            meta_data.size,
            Timestamp(meta_data.created),
            Timestamp(meta_data.modified),
        )
    }
}
