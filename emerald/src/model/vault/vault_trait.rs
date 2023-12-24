use super::{note_types::NoteTypes, Note};

pub trait Vault {
    fn flat_iter(&self) -> std::vec::IntoIter<Note>;
    fn get_links_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>>;
    fn get_backlinks_of(&self, note: &Note) -> Box<dyn Iterator<Item = NoteTypes>>;
}
