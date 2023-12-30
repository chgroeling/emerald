use super::{link_query_result::LinkQueryResult, ExResourceId};

pub trait GetBacklinks {
    fn get_backlinks_of(&self, rid: &ExResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
