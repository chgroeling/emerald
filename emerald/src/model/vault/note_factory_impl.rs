use super::note_factory::NoteFactory;
use super::uid_trait::UidTrait;
use super::{MdContentRetriever, Note, NoteMetadataRetriever};
use crate::markdown::{DefaultMarkdownFrontmatterSplitter, MarkdownFrontmatterSplitter};

use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl<U>
where
    U: UidTrait,
{
    metadata_retriever: Rc<dyn NoteMetadataRetriever<U>>,
    content_retriever: Rc<dyn MdContentRetriever<U>>,
}

impl<U> NoteFactoryImpl<U>
where
    U: UidTrait,
{
    pub fn new(
        metadata_retriever: Rc<dyn NoteMetadataRetriever<U>>,
        content_retriever: Rc<dyn MdContentRetriever<U>>,
    ) -> Self {
        Self {
            metadata_retriever,
            content_retriever,
        }
    }
}

impl<U> NoteFactory<U> for NoteFactoryImpl<U>
where
    U: UidTrait,
{
    fn create_note(&self, uid: &U) -> Note<U> {
        let (title, filesystem_md, document_md) = self.metadata_retriever.retrieve(uid);
        let content = self.content_retriever.retrieve(uid);
        let markdown_splitter = DefaultMarkdownFrontmatterSplitter::new();

        let (yaml, markdown) = markdown_splitter.split(content);
        let yaml_str = yaml.unwrap_or("");

        Note::new(
            uid.clone(),
            title,
            yaml_str.to_string(),
            markdown.to_string(),
            filesystem_md,
            document_md,
        )
    }
}
