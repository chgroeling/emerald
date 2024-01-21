use crate::types;

/// This trait is used to query an source id and and return all links which points to this source
pub trait TgtIterRetriever {
    fn retrieve(
        &self,
        src: &types::ResourceId,
    ) -> Option<Box<dyn Iterator<Item = types::Link2Tgt> + 'static>>;
}
