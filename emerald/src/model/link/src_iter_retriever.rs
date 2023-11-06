use crate::types;

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait SrcIterRetriever {
    fn retrieve(
        &self,
        tgt: &types::ResourceId,
    ) -> Option<Box<dyn Iterator<Item = types::LinkFrmSrc>>>;
}
