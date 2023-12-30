use super::{link_query_result::LinkQueryResult, VaultResourceId};

pub trait GetBacklinks {
    fn get_backlinks_of(&self, rid: &VaultResourceId) -> Box<dyn Iterator<Item = LinkQueryResult>>;
}
