use crate::types::{LinkFrmSrc, ResourceId};

/// This trait is used to query an target id for all contained links and their pointing resource ids.
pub trait SrcIterRetriever {
    fn retrieve(&self, tgt: ResourceId) -> Option<std::vec::IntoIter<LinkFrmSrc>>;
}