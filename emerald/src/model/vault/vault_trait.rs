use super::{note_types::NoteTypes, resource_id_trait::ResourceIdTrait, Note};

pub trait Vault<T>
where
    T: ResourceIdTrait,
{
    fn get_note(&self, rid: &T) -> Note;

    /// Returns the resource id of the Note note.
    fn get_resource_id(&self, note: &Note) -> Option<&T>;

    /// Returns an iterator over links contained in the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>>>;

    /// Returns an iterator over links pointing to the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes<T>>>;
}
