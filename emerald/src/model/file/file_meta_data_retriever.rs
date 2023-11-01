use crate::types;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait FileMetaDataRetriever {
    fn retrieve(&self, tgt: &types::ResourceId) -> &types::MetaData;
}