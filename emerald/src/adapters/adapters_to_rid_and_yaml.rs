use crate::markdown;
use crate::types;

pub fn adapter_to_rid_and_yaml<'a, I: markdown::MarkdownFrontmatterSplitter + Copy>(
    it_src: impl IntoIterator<Item = (types::ResourceId, &'a str)>,
    splitter: I,
) -> impl Iterator<Item = (types::ResourceId, &'a str)> {
    let it1 = it_src.into_iter().map(move |f| {
        let (yaml, _) = splitter.split(&f.1);
        (f.0, yaml)
    });
    it1.map(move |f| (f.0, splitter.trim_pre_and_postamble(f.1)))
}
