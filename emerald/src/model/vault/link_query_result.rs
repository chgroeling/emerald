use super::VaultResourceId;

pub enum LinkQueryResult<T>
where
    T: std::fmt::Debug + std::hash::Hash + Eq + Clone,
{
    LinkToNote(VaultResourceId<T>),
    LinkToResource(VaultResourceId<T>),
}
