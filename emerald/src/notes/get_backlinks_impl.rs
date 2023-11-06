use super::get_backlinks::GetBacklinks;
use super::Note;
use crate::model::link;
use crate::types;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetBacklinksImpl {
    src_link_retriever: Rc<dyn link::SrcIterRetriever>,
}

impl GetBacklinksImpl {
    pub fn new(src_link_retriever: Rc<dyn link::SrcIterRetriever>) -> Self {
        Self { src_link_retriever }
    }
}
impl GetBacklinks for GetBacklinksImpl {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = types::ResourceId>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.src_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            let valid_src = i.src;
            Some(valid_src)
        }))
    }
}
