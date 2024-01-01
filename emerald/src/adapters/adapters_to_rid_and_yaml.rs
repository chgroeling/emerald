use crate::markdown;
use crate::types;

pub fn adapter_to_rid_and_yaml<'a, I: markdown::MarkdownAnalyzer<'a> + Copy>(
    it_src: impl IntoIterator<Item = (types::ResourceId, &'a str)>,
    md_analyzer: I,
) -> impl Iterator<Item = (types::ResourceId, &'a str)> {
    let it1 = it_src.into_iter().map(move |f| {
        let first_element = md_analyzer.analyze(&f.1).next();
        if let Some(types::MdBlock::YamlFrontmatter(yaml)) = first_element {
            // markdown starts when yaml ends
            (f.0, yaml)
        } else {
            (f.0, "")
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
