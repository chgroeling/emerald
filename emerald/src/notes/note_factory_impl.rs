use super::note_factory::NoteFactory;
use super::Note;
use crate::model::content;
use crate::model::note;
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
        Note::new(
            rid,
            meta_data.title.clone(),
            content.0.clone(),
            meta_data.size,
            meta_data.created,
            meta_data.modified,
        )
    }
}
