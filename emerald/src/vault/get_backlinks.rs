use super::{link_query_result::LinkQueryResult, Note};

pub trait GetBacklinks {
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
