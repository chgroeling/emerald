use super::resource_id_trait::ResourceIdTrait;
use super::uid::Uid;
use std::collections::HashMap;

/// Manages mappings between UIDs (unique identifiers) and resource IDs.
#[derive(Debug, Clone)]
pub struct UidMap<T>
where
    T: ResourceIdTrait,
{
    uid_to_rid: HashMap<Uid, T>,
    rid_to_uid: HashMap<T, Uid>,
    next_uid: u32,
}

impl<T> UidMap<T>
where
    T: ResourceIdTrait,
{
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

    pub fn get_rid_from_uid(&self, uid: &Uid) -> Option<&T> {
        self.uid_to_rid.get(uid)
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
    pub fn get_uid_from_rid(&self, rid: &T) -> Option<&Uid> {
        self.rid_to_uid.get(rid)
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
    pub fn assign_uid(&mut self, rid: &T) -> Uid {
        let act_uid = self.next_uid;
        let uid = Uid(act_uid.to_string().into_boxed_str());
        self.rid_to_uid.insert(rid.clone(), uid.clone());
        self.uid_to_rid.insert(uid.clone(), rid.clone());
        self.next_uid += 1;
        uid
    }
}
