use std::rc::Rc;

use crate::{
    model::{unique_id, vault},
    types,
};

pub struct UidRetrieverAdapter {
    uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>,
}

impl UidRetrieverAdapter {
    pub fn new(uid_retriever: Rc<dyn unique_id::UidRetriever<types::ResourceId>>) -> Self {
        Self { uid_retriever }
    }
}
impl vault::UidRetriever<types::ResourceId, unique_id::Uid> for UidRetrieverAdapter {
    fn get_uid_from_rid(&self, rid: &types::ResourceId) -> Option<&unique_id::Uid> {
        self.uid_retriever.get_uid_from_rid(rid)
    }

    fn get_rid_from_uid(&self, uid: &unique_id::Uid) -> Option<&types::ResourceId> {
        self.uid_retriever.get_rid_from_uid(uid)
    }
}
