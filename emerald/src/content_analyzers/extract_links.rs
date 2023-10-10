#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_type::ContentType;
use crate::types::link::Link;

pub fn extract_links(
    iter: impl Iterator<Item = ContentType> + 'static,
) -> impl Iterator<Item = Link> + 'static {
    fn filter_func(pred: &ContentType) -> bool {
        matches!(pred, ContentType::WikiLink(_))
    }

    fn map_func(x: ContentType) -> Link {
        match x {
            ContentType::WikiLink(link) => Link(link),
            _ => panic!(),
        }
    }
    iter.filter(filter_func).map(map_func)
}
