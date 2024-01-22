use super::uid_trait::UidTrait;

pub trait MdContentRetriever<U>
where
    U: UidTrait,
{
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `uid`: A reference to a valid type which holds an UIDS
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, uid: &U) -> &str;
}
