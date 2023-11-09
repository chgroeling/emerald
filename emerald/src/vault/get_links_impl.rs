use super::note::Note;
use super::{get_links::GetLinks, link_query_result::LinkQueryResult};
use crate::model::resource::ResourceMetaDataRetriever;
use crate::model::{link, resource};
use crate::types;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetLinksImpl<I = LinkQueryResultBuilderImpl> {
    tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
    res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    pd: PhantomData<I>,
}

#[derive(Clone)]
pub struct LinkQueryResultBuilderImpl;

trait LinkQueryResultBuilder {
    fn convert_to_link_query_result(
        res_meta_data_retriever: &dyn ResourceMetaDataRetriever,
        rid: types::ResourceId,
    ) -> LinkQueryResult {
        let rmd = res_meta_data_retriever.retrieve(&rid);
        match rmd.resource_type {
            crate::types::ResourceType::Unknown() => LinkQueryResult::LinkToResource(rid),
            crate::types::ResourceType::Markdown() => LinkQueryResult::LinkToNote(rid),
            crate::types::ResourceType::NoType() => LinkQueryResult::LinkToResource(rid),
        }
    }
}

impl LinkQueryResultBuilder for LinkQueryResultBuilderImpl {}

impl<I> GetLinksImpl<I> {
    pub fn new(
        tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    ) -> Self {
        Self {
            tgt_link_retriever,
            res_meta_data_ret,
            pd: Default::default(),
        }
    }
}

impl<I> GetLinks for GetLinksImpl<I>
where
    I: LinkQueryResultBuilder,
{
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let res_meta_data_ret = self.res_meta_data_ret.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            if let Some(valid_tgt) = i.tgt {
                Some(I::convert_to_link_query_result(
                    res_meta_data_ret.as_ref(),
                    valid_tgt,
                ))
            } else {
                None
            }
        }))
    }
}
