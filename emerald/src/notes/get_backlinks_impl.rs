use super::get_backlinks::GetBacklinks;
use super::{Note, NoteFactory};
use crate::model::note;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetBacklinksImpl {
    note_factory: Rc<dyn NoteFactory>,
    src_link_retriever: Rc<dyn note::SrcIterRetriever>,
}

impl GetBacklinksImpl {
    pub fn new(
        note_factory: Rc<dyn NoteFactory>,
        src_link_retriever: Rc<dyn note::SrcIterRetriever>,
    ) -> Self {
        Self {
            note_factory,
            src_link_retriever,
        }
    }
}
impl GetBacklinks for GetBacklinksImpl {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = Note>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.src_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let factory_clone = self.note_factory.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            let valid_src = i.src;
            Some(factory_clone.create_note(valid_src))
        }))
    }
}
