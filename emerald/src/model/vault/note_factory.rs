use super::Note;
use crate::model::unique_id;

pub trait NoteFactory {
    fn create_note(&self, uid: &unique_id::Uid) -> Note;
}
