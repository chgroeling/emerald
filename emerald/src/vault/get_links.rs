use super::{link_query_result::LinkQueryResult, Note};

pub trait GetLinks {
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
