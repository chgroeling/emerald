use super::note_factory::NoteFactory;
use super::resource_id::ResourceId;
use super::{Note, NoteMetadataRetriever};
use crate::markdown::MarkdownFrontMatterSplitter;
use crate::model::content;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    metadata_retriever: Rc<dyn NoteMetadataRetriever>,
    content_retriever: Rc<dyn content::MdContentRetriever>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn NoteMetadataRetriever>,
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
        let rid_conv: ResourceId = rid.clone().into();
        let (title, filesystem_md, document_md) = self.metadata_retriever.retrieve(&rid_conv);

        let content = self.content_retriever.retrieve(&rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

        Note::new(
            rid.into(),
            title,
            yaml_str.to_string(),
            markdown.to_string(),
            filesystem_md,
            document_md,
        )
    }
}
