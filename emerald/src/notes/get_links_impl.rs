use super::get_links::{GetLinks, GetLinksResult};
use super::Note;
use crate::model::{file, link, note};
use std::rc::Rc;

#[derive(Clone)]
pub struct GetLinksImpl {
    tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
    meta_data_retriever: Rc<dyn file::FileMetaDataRetriever>,
}

impl GetLinksImpl {
    pub fn new(
        tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
        meta_data_retriever: Rc<dyn file::FileMetaDataRetriever>,
    ) -> Self {
        Self {
            tgt_link_retriever,
            meta_data_retriever,
        }
    }
}
impl GetLinks for GetLinksImpl {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = GetLinksResult>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                Some(GetLinksResult::LinkToNote(valid_tgt))
            } else {
                None
            }
        }))
    }
}
