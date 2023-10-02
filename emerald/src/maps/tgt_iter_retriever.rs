use crate::types::{Link2Tgt, ResourceId};

/// This trait is used to query an source id and and return all links which points to this source
pub trait TgtIterRetriever {
    fn retrieve(&self, src: ResourceId) -> Option<std::vec::IntoIter<Link2Tgt>>;
}
