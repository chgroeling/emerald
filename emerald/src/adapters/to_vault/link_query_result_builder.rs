use crate::model::resource::ResourceMetadataRetriever;
use crate::model::vault;
use crate::types;

#[derive(Clone)]
pub struct LinkQueryResultBuilderImpl;

pub trait LinkQueryResultBuilder {
    fn convert_to_link_query_result(
        res_meta_data_retriever: &dyn ResourceMetadataRetriever,
        rid: types::ResourceId,
    ) -> vault::LinkQueryResult<vault::ExResourceId> {
        let rmd = res_meta_data_retriever.retrieve(&rid);
        match rmd.resource_type {
            crate::types::ResourceType::Unknown() => {
                vault::LinkQueryResult::LinkToResource(vault::VaultResourceId(rid.into()))
            }
            crate::types::ResourceType::Markdown() => {
                vault::LinkQueryResult::LinkToNote(vault::VaultResourceId(rid.into()))
            }
            crate::types::ResourceType::NoType() => {
                vault::LinkQueryResult::LinkToResource(vault::VaultResourceId(rid.into()))
            }
        }
    }
}

impl LinkQueryResultBuilder for LinkQueryResultBuilderImpl {}
