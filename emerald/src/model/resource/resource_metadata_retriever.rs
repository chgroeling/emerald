use super::resource_metadata::ResourceMetadata;
use crate::types;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait ResourceMetadataRetriever {
    fn retrieve(&self, tgt: &types::ResourceId) -> &ResourceMetadata;
}
