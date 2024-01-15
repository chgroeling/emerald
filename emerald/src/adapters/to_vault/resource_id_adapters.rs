use crate::types;

/// Converts an iterator of `types::ResourceId` to `types::ResourceId`.
///
/// # Arguments
///
/// * `it_src` - An iterator of `types::ResourceId`.
///
/// # Returns
///
/// Iterator over `types::ResourceId`.
pub fn convert_resource_ids_to_vault_format<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
) -> impl Iterator<Item = types::ResourceId> + 'a {
    it_src.into_iter().map(|f| f.into())
}
