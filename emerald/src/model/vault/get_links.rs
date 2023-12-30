use super::{link_query_result::LinkQueryResult, ExResourceId};

pub trait GetLinks {
    fn get_links_of(&self, rid: &ExResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
