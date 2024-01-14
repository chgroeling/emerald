use crate::model::vault;
use crate::types;

/// Converts an iterator of `types::ResourceId` to `vault::ExResourceId`.
///
/// # Arguments
///
/// * `it_src` - An iterator of `types::ResourceId`.
///
/// # Returns
///
/// Iterator over `vault::ExResourceId`.
pub fn convert_resource_ids_to_vault_format<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
) -> impl Iterator<Item = vault::VaultResourceId<vault::ExResourceId>> + 'a {
    it_src
        .into_iter()
        .map(|f| vault::VaultResourceId::<vault::ExResourceId>(f.into()))
}
