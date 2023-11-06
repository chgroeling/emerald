use super::get_links::GetLinks;
use super::{Note, NoteFactory};
use crate::model::note;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetLinksImpl {
    note_factory: Rc<dyn NoteFactory>,
    tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
}

impl GetLinksImpl {
    pub fn new(
        note_factory: Rc<dyn NoteFactory>,
        tgt_link_retriever: Rc<dyn note::TgtIterRetriever>,
    ) -> Self {
        Self {
            note_factory,
            tgt_link_retriever,
        }
    }
}
impl GetLinks for GetLinksImpl {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let factory_clone = self.note_factory.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                Some(factory_clone.create_note(valid_tgt))
            } else {
                None
            }
        }))
    }
}
