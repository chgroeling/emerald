use super::link_query_result::LinkQueryResult;
use crate::model::resource::ResourceMetaDataRetriever;
use crate::types;

#[derive(Clone)]
pub struct LinkQueryResultBuilderImpl;

pub trait LinkQueryResultBuilder {
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
