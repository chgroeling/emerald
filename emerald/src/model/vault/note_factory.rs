use super::{Note, ResourceId};

pub trait NoteFactory {
    fn create_note(&self, rid: ResourceId) -> Note;
}
