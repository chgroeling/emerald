use crate::model::vault;
use crate::types;

pub fn adapter_to_vault_rid<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
) -> impl Iterator<Item = vault::ResourceId> + 'a {
    it_src.into_iter().map(|f| f.into())
}
