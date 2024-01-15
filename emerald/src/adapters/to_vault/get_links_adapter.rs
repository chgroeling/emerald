use super::link_query_result_builder::{LinkQueryResultBuilder, LinkQueryResultBuilderImpl};
use crate::model::vault;
use crate::model::{link, resource};
use crate::types;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone)]
pub struct GetLinksAdapter<I = LinkQueryResultBuilderImpl> {
    tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
    res_meta_data_ret: Rc<dyn resource::ResourceMetadataRetriever>,
    pd: PhantomData<I>,
}

impl GetLinksAdapter {
    pub fn new(
        tgt_link_retriever: Rc<dyn link::TgtIterRetriever>,
        res_meta_data_ret: Rc<dyn resource::ResourceMetadataRetriever>,
    ) -> Self {
        Self {
            tgt_link_retriever,
            res_meta_data_ret,
            pd: Default::default(),
        }
    }
}

impl<I> vault::GetLinks<types::ResourceId> for GetLinksAdapter<I>
where
    I: LinkQueryResultBuilder,
{
    fn get_links_of(
        &self,
        rid: &types::ResourceId,
    ) -> Box<dyn Iterator<Item = vault::LinkQueryResult<types::ResourceId>>> {
        let rid: types::ResourceId = rid.clone().0.into();
        let Some(out_itr) = self.tgt_link_retriever.retrieve(&rid) else {
            return Box::new(std::iter::empty());
        };
        let res_meta_data_ret = self.res_meta_data_ret.clone();
        Box::new(out_itr.filter_map(move |i| {
            // only consider valid targets
            i.tgt.map(|valid_tgt| {
                I::convert_to_link_query_result(res_meta_data_ret.as_ref(), valid_tgt)
            })
        }))
    }
}
