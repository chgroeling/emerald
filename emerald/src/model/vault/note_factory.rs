use super::Note;
use crate::model::uid;

pub trait NoteFactory {
    fn create_note(&self, uid: &uid::Uid) -> Note;
}
