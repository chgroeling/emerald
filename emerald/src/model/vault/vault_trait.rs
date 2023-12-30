use super::{note_types::NoteTypes, Note};

pub trait Vault {
    fn flat_iter(&self) -> std::vec::IntoIter<Note>;

    /// Returns an iterator over links contained in the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>>;

    /// Returns an iterator over links pointing to the specified Note.
    ///
    /// # Arguments
    ///
    /// * `note`: Note.
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>>;
}
