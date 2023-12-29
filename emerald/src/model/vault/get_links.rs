use super::{link_query_result::LinkQueryResult, ResourceId};

pub trait GetLinks {
    fn get_links_of(&self, rid: &ResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
