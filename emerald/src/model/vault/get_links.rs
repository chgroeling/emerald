use super::{link_query_result::LinkQueryResult, VaultResourceId};

pub trait GetLinks {
    fn get_links_of(&self, rid: &VaultResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
