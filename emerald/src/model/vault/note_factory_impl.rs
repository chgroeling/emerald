use super::note_factory::NoteFactory;
use super::uid_map::UidMap;
use super::{MdContentRetriever, Note, NoteMetadataRetriever, Uid};
use crate::markdown::MarkdownFrontMatterSplitter;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteFactoryImpl {
    metadata_retriever: Rc<dyn NoteMetadataRetriever>,
    content_retriever: Rc<dyn MdContentRetriever>,
    uid_map: Rc<UidMap>,
}

impl NoteFactoryImpl {
    pub fn new(
        meta_data_retriever: Rc<dyn NoteMetadataRetriever>,
        content_retriever: Rc<dyn MdContentRetriever>,
        uid_map: Rc<UidMap>,
    ) -> Self {
        Self {
            metadata_retriever: meta_data_retriever,
            content_retriever,
            uid_map,
        }
    }
}

impl NoteFactory for NoteFactoryImpl {
    fn create_note(&self, uid: &Uid) -> Note {
        let rid = self.uid_map.get_rid_from_uid(uid).expect("Should exist");
        let (title, filesystem_md, document_md) = self.metadata_retriever.retrieve(rid);
        let content = self.content_retriever.retrieve(rid);
        let markdown_splitter = MarkdownFrontMatterSplitter::new();

        let (yaml_str, markdown) = markdown_splitter.split(content);

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
