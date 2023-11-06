use super::get_backlinks::GetBacklinks;
use super::link_query_result::LinkQueryResult;
use super::Note;
use crate::model::{link, resource};
use std::rc::Rc;

#[derive(Clone)]
pub struct GetBacklinksImpl {
    src_link_retriever: Rc<dyn link::SrcIterRetriever>,
    res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
}

impl GetBacklinksImpl {
    pub fn new(
        src_link_retriever: Rc<dyn link::SrcIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    ) -> Self {
        Self {
            src_link_retriever,
            res_meta_data_ret,
        }
    }
}
impl GetBacklinks for GetBacklinksImpl {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.src_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let res_meta_data_ret = self.res_meta_data_ret.clone();
        Box::new(out_itr.map(move |i| {
            // only consider valid targets
            let valid_src = i.src;

            let rmd = res_meta_data_ret.retrieve(&valid_src);
            match rmd.resource_type {
                crate::types::ResourceType::Unknown() => LinkQueryResult::LinkToResource(valid_src),
                crate::types::ResourceType::Markdown() => LinkQueryResult::LinkToNote(valid_src),
                crate::types::ResourceType::NoType() => LinkQueryResult::LinkToResource(valid_src),
            }
        }))
    }
}
