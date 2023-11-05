use super::Provider;
use crate::model::note;
use crate::notes;

use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct BacklinkProvider {
    note_factory: Rc<dyn notes::NoteFactory>,
    tgt_link_retriever: Rc<dyn note::SrcIterRetriever>,
}

impl BacklinkProvider {
    pub fn new(
        note_factory: Rc<dyn notes::NoteFactory>,
        tgt_link_retriever: Rc<dyn note::SrcIterRetriever>,
    ) -> Self {
        Self {
            note_factory,
            tgt_link_retriever,
        }
    }
}

impl Provider<Box<dyn Iterator<Item = notes::Note>>> for BacklinkProvider {
    fn get(&self, rid: &types::ResourceId) -> Box<dyn Iterator<Item = notes::Note>> {
        let Some(out_itr) = self.tgt_link_retriever.retrieve(rid) else {
            return Box::new(std::iter::empty());
        };
        let self_clone = self.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            let valid_tgt = i.src;
            Some(self_clone.note_factory.create_note(valid_tgt))
        }))
    }
}
