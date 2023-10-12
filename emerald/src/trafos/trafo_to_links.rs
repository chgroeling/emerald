#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{md_analyzer::ContentType, types::link::Link};

pub fn trafo_to_links<'a>(
    iter: impl Iterator<Item = ContentType> + 'a,
) -> impl Iterator<Item = Link> + 'a {
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
