#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::ContentType;
use crate::types::Link;

pub fn adapter_from_content_type_to_links<'a>(
    it_src: impl IntoIterator<Item = ContentType<'a>> + 'a,
) -> impl Iterator<Item = Link> + 'a {
    fn filter_func(pred: &ContentType) -> bool {
        matches!(pred, ContentType::WikiLink(_))
    }

    fn map_func(x: ContentType) -> Link {
        match x {
            ContentType::WikiLink(link) => Link(link.to_owned()),
            _ => panic!(),
        }
    }
    it_src.into_iter().filter(filter_func).map(map_func)
}
