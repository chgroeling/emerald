use std::collections::HashMap;

use super::ex_resource_id::VaultResourceId;
use super::uid::Uid;
use super::ExResourceId;

/// Manages mappings between UIDs (unique identifiers) and resource IDs.
#[derive(Debug, Clone)]
pub struct UidMap {
    uid_to_rid: HashMap<Uid, VaultResourceId<ExResourceId>>,
    rid_to_uid: HashMap<VaultResourceId<ExResourceId>, Uid>,
    next_uid: u32,
}

impl UidMap {
    /// Constructs a new `UidMap`.
    pub fn new() -> Self {
        Self {
            uid_to_rid: HashMap::new(),
            rid_to_uid: HashMap::new(),
            next_uid: 0,
        }
    }
    /// Retrieves the resource ID associated with a given UID.
    ///
    /// # Arguments
    ///
    /// * `uid`: The UID to look up.
    ///
    /// # Returns
    ///
    /// Option containing the corresponding resource ID, if it exists.

    pub fn get_rid_from_uid(&self, uid: &Uid) -> Option<&ExResourceId> {
        self.uid_to_rid.get(uid).map(|f| &f.0)
    }

    /// Retrieves the UID associated with a given resource ID.
    ///
    /// # Arguments
    ///
    /// * `rid`: The resource ID to look up.
    ///
    /// # Returns
    ///
    /// Option containing the corresponding UID, if it exists.
    pub fn get_uid_from_rid(&self, rid: &ExResourceId) -> Option<&Uid> {
        let vrid = VaultResourceId::<ExResourceId>(rid.clone());
        self.rid_to_uid.get(&vrid)
    }

    /// Assigns a new UID to the given resource ID.
    ///
    /// # Arguments
    ///
    /// * `rid`: The resource ID to assign a UID.
    ///
    /// # Returns
    ///
    /// The new UID assigned to the resource ID.
    pub fn assign_uid(&mut self, rid: &VaultResourceId<ExResourceId>) -> Uid {
        let act_uid = self.next_uid;
        let uid = Uid(act_uid.to_string().into_boxed_str());
        self.rid_to_uid.insert(rid.clone(), uid.clone());
        self.uid_to_rid.insert(uid.clone(), rid.clone());
        self.next_uid += 1;
        uid
    }
}
