use super::resource_ref::ResourceRef;
use super::Note;

pub enum NoteTypes {
    Note(Note),
    ResourceRef(ResourceRef),
}
