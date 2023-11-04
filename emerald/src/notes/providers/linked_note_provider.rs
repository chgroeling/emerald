use super::Provider;
use crate::model::note;
use crate::notes;

use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct LinkedNoteProvider {
    note_factory: Rc<dyn notes::NoteFactory>,
    tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
}

impl LinkedNoteProvider {
    pub fn new(
        note_factory: Rc<dyn notes::NoteFactory>,
        tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
    ) -> Self {
        Self {
            note_factory,
            tgt_link_retriever,
        }
    }
}

impl Provider<Box<dyn Iterator<Item = notes::Note>>> for LinkedNoteProvider {
    fn get(&self, rid: &types::ResourceId) -> Box<dyn Iterator<Item = notes::Note>> {
        let Some(out_itr) = self.tgt_link_retriever.retrieve(rid) else {
            return Box::new(std::iter::empty());
        };
        let self_clone = self.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                Some(self_clone.note_factory.create_note(valid_tgt))
            } else {
                None
            }
        }))
    }
}
