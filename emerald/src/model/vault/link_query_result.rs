use super::ex_resource_id::VaultResourceIdTrait;

pub enum LinkQueryResult<T>
where
    T: VaultResourceIdTrait,
{
    LinkToNote(T),
    LinkToResource(T),
}
