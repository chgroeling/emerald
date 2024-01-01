use super::ExResourceId;

pub trait MdContentRetriever {
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `rid`: A reference to a `note_updater::ExResourceId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, rid: &ExResourceId) -> &str;
}
