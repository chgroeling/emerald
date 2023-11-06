use super::Note;
use crate::types;

pub trait NoteFactory {
    fn create_note(&self, rid: types::ResourceId) -> Note;
}
