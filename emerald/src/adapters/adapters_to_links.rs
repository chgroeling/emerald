use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn adapter_from_content_type_to_links<'a>(
    it_src: impl IntoIterator<Item = types::ContentType<'a>> + 'a,
) -> impl Iterator<Item = types::Link> + 'a {
    fn filter_func(pred: &types::ContentType) -> bool {
        matches!(pred, types::ContentType::WikiLink(_))
    }

    fn map_func(x: types::ContentType) -> types::Link {
        match x {
            types::ContentType::WikiLink(link) => types::Link(link.to_owned()),
            _ => panic!(),
        }
    }
    it_src.into_iter().filter(filter_func).map(map_func)
}
