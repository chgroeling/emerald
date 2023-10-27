use crate::{markdown, types};

pub fn adapter_analyze_md<'a, I: markdown::MarkdownAnalyzer<'a> + 'a + Copy>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::Content)> + 'a,
    md_analyzer: I,
) -> impl Iterator<Item = (&'a types::ResourceId, types::ContentType<'a>)> + 'a {
    it_src
        .into_iter()
        .flat_map(move |(rid, content)| md_analyzer.analyze(&content.0).map(move |f| (rid, f)))
}
