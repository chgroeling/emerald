use super::{vault_resource_id_trait::VaultResourceIdTrait, Note};

pub enum NoteTypes<T>
where
    T: VaultResourceIdTrait,
{
    Note(Note),
    ResourceRef(T),
}
