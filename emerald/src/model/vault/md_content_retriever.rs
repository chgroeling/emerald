use super::VaultResourceId;

pub trait MdContentRetriever<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `rid`: A reference to a `vault::ExResourceId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, rid: &VaultResourceId<T>) -> &str;
}
