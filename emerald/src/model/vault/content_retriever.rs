use super::ExResourceId;

pub trait ContentRetriever {
    /// Retrieves content for the specified resource identifier.
    ///
    /// # Arguments
    ///
    /// * `rid`: A reference to a `vault::ExResourceId`.
    ///
    /// # Returns
    ///
    /// A string slice containing the retrieved content.
    fn retrieve(&self, rid: &ExResourceId) -> &str;
}
