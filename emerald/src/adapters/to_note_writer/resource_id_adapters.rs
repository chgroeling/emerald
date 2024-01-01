use crate::model::note_writer;
use crate::types;

/// Converts an iterator of `types::ResourceId` to `note_writer::ExResourceId`.
///
/// # Arguments
///
/// * `it_src` - An iterator of `types::ResourceId`.
///
/// # Returns
///
/// Iterator over `note_writer::ExResourceId`.
pub fn convert_resource_ids_to_note_writer_format<'a>(
    it_src: impl IntoIterator<Item = types::ResourceId> + 'a,
) -> impl Iterator<Item = note_writer::ExResourceId> + 'a {
    it_src.into_iter().map(|f| f.into())
}
