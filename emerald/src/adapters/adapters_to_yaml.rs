use crate::markdown;
use crate::types;

pub fn adapter_to_yaml<'a, I: markdown::MarkdownAnalyzer<'a> + 'a + Copy>(
    it_src: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::Content)> + 'a,
    md_analyzer: I,
) -> impl Iterator<Item = (&'a types::ResourceId, &'a str)> + 'a {
    let it1 = it_src.into_iter().filter_map(move |f| {
        let first_element = md_analyzer.analyze(&f.1 .0).next();
        if let Some(types::MdBlock::YamlFrontmatter(yaml)) = first_element {
            // markdown starts when yaml ends
            Some((f.0, yaml))
        } else {
            None
        }
    });

    it1.map(|f| {
        (
            f.0,
            f.1.trim_start_matches("---")
                .trim_start_matches('\n')
                .trim_end_matches('\n')
                .trim_end_matches("---")
                .trim_end_matches('\n'),
        )
    })
}
