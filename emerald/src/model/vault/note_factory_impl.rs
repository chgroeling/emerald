use super::note_factory::NoteFactory;
use super::uid_map::UidMap;
use super::vault_resource_id_trait::VaultResourceIdTrait;
use super::{MdContentRetriever, Note, NoteMetadataRetriever, Uid};
use crate::markdown::{DefaultMarkdownFrontmatterSplitter, MarkdownFrontmatterSplitter};
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl<T>
where
    T: VaultResourceIdTrait,
{
    metadata_retriever: Rc<dyn NoteMetadataRetriever<T>>,
    content_retriever: Rc<dyn MdContentRetriever<T>>,
    uid_map: Rc<UidMap<T>>,
}

impl<T> NoteFactoryImpl<T>
where
    T: VaultResourceIdTrait,
{
    pub fn new(
        meta_data_retriever: Rc<dyn NoteMetadataRetriever<T>>,
        content_retriever: Rc<dyn MdContentRetriever<T>>,
        uid_map: Rc<UidMap<T>>,
    ) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
            content_retriever,
            uid_map,
        }
    }
}

impl<T> NoteFactory for NoteFactoryImpl<T>
where
    T: VaultResourceIdTrait,
{
    fn create_note(&self, uid: &Uid) -> Note {
        let rid = self.uid_map.get_rid_from_uid(uid).expect("Should exist");
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
