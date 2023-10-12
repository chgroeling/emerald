use crate::types::{meta_data::FileType, ResourceId};

pub fn filter_markdown_types<'a>(
    iter: impl Iterator<Item = (FileType, ResourceId)> + 'a,
) -> impl Iterator<Item = ResourceId> + 'a {
    iter.filter(|pred| matches!(pred.0, FileType::Markdown(_)))
        .map(|f| f.1)
}
