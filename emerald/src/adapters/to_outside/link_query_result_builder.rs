use crate::model::resource;
use crate::types;

use super::LinkQueryResult;

#[derive(Clone)]
pub struct LinkQueryResultBuilderImpl;

pub trait LinkQueryResultBuilder {
    fn convert_to_link_query_result(
        res_meta_data_retriever: &dyn resource::ResourceMetadataRetriever,
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
