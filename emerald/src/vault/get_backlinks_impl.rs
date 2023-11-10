use super::link_query_result::LinkQueryResult;
use super::link_query_result_builder::LinkQueryResultBuilderImpl;
use super::note::Note;
use super::{get_backlinks::GetBacklinks, link_query_result_builder::LinkQueryResultBuilder};
use crate::model::{link, resource};
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetBacklinksImpl<I = LinkQueryResultBuilderImpl> {
    src_link_retriever: Rc<dyn link::SrcIterRetriever>,
    res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    pd: PhantomData<I>,
}

impl GetBacklinksImpl {
    pub fn new(
        src_link_retriever: Rc<dyn link::SrcIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetaDataRetriever>,
    ) -> Self {
        Self {
            src_link_retriever,
            res_meta_data_ret,
            pd: Default::default(),
        }
    }
}
impl<I> GetBacklinks for GetBacklinksImpl<I>
where
    I: LinkQueryResultBuilder,
{
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>> {
        let rid = note.rid.clone();
        let Some(out_itr) = self.src_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let res_meta_data_ret = self.res_meta_data_ret.clone();
        Box::new(out_itr.map(move |i| {
            // only consider valid targets
            I::convert_to_link_query_result(res_meta_data_ret.as_ref(), i.src)
        }))
    }
}