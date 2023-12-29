use super::{link_query_result::LinkQueryResult, ResourceId};

pub trait GetBacklinks {
    fn get_backlinks_of(&self, rid: &ResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
