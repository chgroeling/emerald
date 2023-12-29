use super::{Note, Uid};

pub trait NoteFactory {
    fn create_note(&self, uid: &Uid) -> Note;
}
