use super::{ex_resource_id::VaultResourceIdTrait, VaultResourceId};

pub enum LinkQueryResult<T>
where
    T: VaultResourceIdTrait,
{
    LinkToNote(VaultResourceId<T>),
    LinkToResource(VaultResourceId<T>),
}
