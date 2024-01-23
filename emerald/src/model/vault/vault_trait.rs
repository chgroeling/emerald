use super::{note_types::NoteTypes, resource_id_trait::ResourceIdTrait, uid_trait::UidTrait, Note};

pub trait Vault<T, U>
where
    T: ResourceIdTrait,
    U: UidTrait,
{
    fn get_note(&self, uid: &U) -> Note<U>;

    /// Returns the resource id of the Note note.
    fn get_resource_id(&self, note: &Note<U>) -> Option<&T>;

    /// Returns an iterator over links contained in the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_links_of(&self, note: &Note<U>) -> Box<dyn Iterator<Item = NoteTypes<T, U>> + 'static>;

    /// Returns an iterator over links pointing to the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_backlinks_of(
        &self,
        note: &Note<U>,
    ) -> Box<dyn Iterator<Item = NoteTypes<T, U>> + 'static>;
}
