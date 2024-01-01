use crate::model::note_updater;
use crate::types;

/// Converts an iterator of `types::ResourceId` to `note_updater::ExResourceId`.
///
/// # Arguments
///
/// * `it_src` - An iterator of `types::ResourceId`.
///
/// # Returns
///
/// Iterator over `note_updater::ExResourceId`.
pub fn convert_resource_ids_to_note_updater_format<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
) -> impl Iterator<Item = note_updater::ExResourceId> + 'a {
    it_src.into_iter().map(|f| f.into())
}
