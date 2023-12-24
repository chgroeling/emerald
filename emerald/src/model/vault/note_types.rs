use super::{Note, ResourceId};

pub enum NoteTypes {
    Note(Note),
    ResourceRef(ResourceId),
}
