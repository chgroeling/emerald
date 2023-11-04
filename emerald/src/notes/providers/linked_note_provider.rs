use super::Provider;
use crate::model::content;
use crate::model::note;
use crate::notes;
use crate::types;
use std::rc::Rc;

pub struct LinkedNoteProvider {
    note_factory: Box<dyn notes::NoteFactory>,
    meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
}

impl LinkedNoteProvider {
    pub fn new(
        note_factory: Box<dyn notes::NoteFactory>,
        meta_data_retriever: Rc<dyn note::NoteMetaDataRetriever>,
    ) -> Self {
        Self {
            note_factory,
            meta_data_retriever,
        }
    }
}

impl Provider<notes::Note> for LinkedNoteProvider {
    fn get(&self, rid: &types::ResourceId) -> notes::Note {
        self.note_factory.create_note(rid.clone())
    }
}
