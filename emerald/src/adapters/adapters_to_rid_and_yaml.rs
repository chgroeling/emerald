use crate::markdown;
use crate::types;

pub fn adapter_to_rid_and_yaml<'a, I: markdown::MarkdownFrontmatterSplitter + Copy>(
    it_src: impl IntoIterator<Item = (types::ResourceId, &'a str)>,
    splitter: I,
) -> impl Iterator<Item = (types::ResourceId, Option<&'a str>)> {
    it_src.into_iter().map(move |f| {
        let (yaml, _) = splitter.split(f.1);
        (f.0, yaml)
    })
}
