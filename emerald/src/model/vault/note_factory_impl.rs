use super::note_factory::NoteFactory;
use super::resource_id_trait::ResourceIdTrait;
use super::uid_retriever::UidRetriever;
use super::uid_trait::UidTrait;
use super::{MdContentRetriever, Note, NoteMetadataRetriever};
use crate::markdown::{DefaultMarkdownFrontmatterSplitter, MarkdownFrontmatterSplitter};

use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    metadata_retriever: Rc<dyn NoteMetadataRetriever<T>>,
    content_retriever: Rc<dyn MdContentRetriever<T>>,
    uid_retriever: Rc<dyn UidRetriever<T, U>>,
}

impl<T, U> NoteFactoryImpl<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    pub fn new(
        meta_data_retriever: Rc<dyn NoteMetadataRetriever<T>>,
        content_retriever: Rc<dyn MdContentRetriever<T>>,
        uid_retriever: Rc<dyn UidRetriever<T, U>>,
    ) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
            content_retriever,
            uid_retriever,
        }
    }
}

impl<T, U> NoteFactory<U> for NoteFactoryImpl<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    fn create_note(&self, uid: &U) -> Note<U> {
        let rid = self
            .uid_retriever
            .get_rid_from_uid(uid)
            .expect("Should exist");
        let (title, filesystem_md, document_md) = self.metadata_retriever.retrieve(rid);
        let content = self.content_retriever.retrieve(rid);
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
