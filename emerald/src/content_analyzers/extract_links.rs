#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use super::content_type::ContentType;
use crate::types::link::Link;

pub fn extract_links(
    content_type_iter: impl Iterator<Item = ContentType>,
) -> impl Iterator<Item = Link> {
    fn filter_func(pred: &ContentType) -> bool {
        matches!(pred, ContentType::WikiLink(_))
    }

    fn map_func(x: ContentType) -> Link {
        match x {
            ContentType::WikiLink(link) => Link(link),
            _ => panic!(),
        }
    }
    content_type_iter.filter(filter_func).map(map_func)
}
