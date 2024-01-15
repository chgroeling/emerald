use super::{ex_resource_id::VaultResourceIdTrait, Note};

pub enum NoteTypes<T>
where
    T: VaultResourceIdTrait,
{
    Note(Note),
    ResourceRef(T),
}
