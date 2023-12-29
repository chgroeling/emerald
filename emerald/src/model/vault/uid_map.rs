use std::collections::HashMap;

use super::uid::Uid;
use super::ResourceId;

#[derive(Debug, Clone)]
pub struct UidMap {
    uid_to_rid: HashMap<Uid, ResourceId>,
    rid_to_uid: HashMap<ResourceId, Uid>,
    next_uid: u32,
}

impl UidMap {
    pub fn new() -> Self {
        Self {
            uid_to_rid: HashMap::new(),
            rid_to_uid: HashMap::new(),
            next_uid: 0,
        }
    }
    pub fn get_rid_from_uid(&self, uid: &Uid) -> Option<&ResourceId> {
        self.uid_to_rid.get(uid)
    }

    pub fn get_uid_from_rid(&self, rid: &ResourceId) -> Option<&Uid> {
        self.rid_to_uid.get(rid)
    }

    pub fn assign_uid(&mut self, rid: &ResourceId) -> Uid {
        let act_uid = self.next_uid;
        let uid = Uid(act_uid.to_string().into_boxed_str());
        self.rid_to_uid.insert(rid.clone(), uid.clone());
        self.uid_to_rid.insert(uid.clone(), rid.clone());
        self.next_uid += 1;
        uid
    }
}
