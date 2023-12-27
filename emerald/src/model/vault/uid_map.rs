use std::collections::HashMap;

use super::uid::Uid;
use super::ResourceId;

pub struct UidMap {
    uid_to_rid: HashMap<Uid, ResourceId>,
    rid_to_uid: HashMap<ResourceId, Uid>,
}

impl UidMap {
    pub fn new() -> Self {
        Self {
            uid_to_rid: HashMap::new(),
            rid_to_uid: HashMap::new(),
        }
    }
    pub fn resource_id_from_uid(&self, uid: &Uid) -> ResourceId {
        todo!();
    }

    pub fn get_or_assign_uid(&mut self, rid: &ResourceId) -> Uid {
        todo!();
    }
}
