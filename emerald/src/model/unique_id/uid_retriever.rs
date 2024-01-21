use super::{resource_id_trait::ResourceIdTrait, Uid};

pub trait UidRetriever<T>
where
    T: ResourceIdTrait,
{
    /// Retrieves the UID associated with a given resource ID.
    ///
    /// # Arguments
    ///
    /// * `rid`: The resource ID to look up.
    ///
    /// # Returns
    ///
    /// Option containing the corresponding UID, if it exists.
    fn get_uid_from_rid(&self, rid: &T) -> Option<&Uid>;

    /// Retrieves the resource ID associated with a given UID.
    ///
    /// # Arguments
    ///
    /// * `uid`: The UID to look up.
    ///
    /// # Returns
    ///
    /// Option containing the corresponding resource ID, if it exists.
    fn get_rid_from_uid(&self, uid: &Uid) -> Option<&T>;
}
