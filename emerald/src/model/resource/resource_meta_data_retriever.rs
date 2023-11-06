use super::resource_meta_data::ResourceMetaData;
use crate::types;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait ResourceMetaDataRetriever {
    fn retrieve(&self, tgt: &types::ResourceId) -> &ResourceMetaData;
}
