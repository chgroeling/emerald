use super::{Note, VaultResourceId};

pub enum NoteTypes {
    Note(Note),
    ResourceRef(VaultResourceId),
}
