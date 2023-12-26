use super::note_factory::NoteFactory;
use super::resource_id::ResourceId;
use super::{ContentRetriever, Note, NoteMetadataRetriever};
use crate::markdown::MarkdownFrontMatterSplitter;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    metadata_retriever: Rc<dyn NoteMetadataRetriever>,
    content_retriever: Rc<dyn ContentRetriever>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn NoteMetadataRetriever>,
        content_retriever: Rc<dyn ContentRetriever>,
    ) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
            content_retriever,
        }
    }
}

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, rid: &ResourceId) -> Note {
        let (title, filesystem_md, document_md) = self.metadata_retriever.retrieve(rid);
        let content = self.content_retriever.retrieve(&rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

        Note::new(
            rid.clone(),
            title,
            yaml_str.to_string(),
            markdown.to_string(),
            filesystem_md,
            document_md,
        )
    }
}
