use super::vault_resource_id_trait::VaultResourceIdTrait;

pub enum LinkQueryResult<T>
where
    T: VaultResourceIdTrait,
{
    LinkToNote(T),
    LinkToResource(T),
}
