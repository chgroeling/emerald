use super::{ExResourceId, Note};

pub enum NoteTypes {
    Note(Note),
    ResourceRef(ExResourceId),
}
