use super::Note;
use super::{get_links::GetLinks, link_query_result::LinkQueryResult};
use crate::model::{link, resource};
use std::rc::Rc;

#[derive(Clone)]
pub struct GetLinksImpl {
    tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
    res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
}

impl GetLinksImpl {
    pub fn new(
        tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    ) -> Self {
        Self {
            tgt_link_retriever,
            res_meta_data_ret,
        }
    }
}
impl GetLinks for GetLinksImpl {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let res_meta_data_ret = self.res_meta_data_ret.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                let rmd = res_meta_data_ret.retrieve(&valid_tgt);
                let ret = match rmd.resource_type {
                    crate::types::ResourceType::Unknown() => {
                        LinkQueryResult::LinkToResource(valid_tgt)
                    }
                    crate::types::ResourceType::Markdown() => {
                        LinkQueryResult::LinkToNote(valid_tgt)
                    }
                    crate::types::ResourceType::NoType() => {
                        LinkQueryResult::LinkToResource(valid_tgt)
                    }
                };
                Some(ret)
            } else {
                None
            }
        }))
    }
}
