#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterable,
    resources::ContentIterSource,
    types::{LinkFromSourceToTarget, LinkToTarget},
};

use super::all_note_links_iterable::AllNoteLinksIterable;

#[allow(dead_code)]
pub type SourceAndLinkToTargetList = Vec<LinkFromSourceToTarget>;

pub struct NoteLinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,
    source_and_link_to_target_list: SourceAndLinkToTargetList,
}

impl NoteLinkIndex {
    pub fn new(
        content_iter_src: &impl ContentIterSource,
        md_link_analyer_iterable: &impl MdLinkAnalyzerIterable,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut source_and_link_to_target_list = SourceAndLinkToTargetList::new();

        for (src, content) in content_iter_src.iter() {
            trace!("Link extraction from {:?} starts", &src);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_to_target in md_link_analyer_iterable.create_iter(content.0.as_ref().clone()) {
                match &link_to_target {
                    LinkToTarget { link, target: None } => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &src, &link);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                let s2t = LinkFromSourceToTarget::from_link_to_target(src.clone(), link_to_target);
                source_and_link_to_target_list.push(s2t);
            }

            if note_valid_backlink_cnt == 0 {
                trace!("No valid links found in  {:?}", &src);
            }

            valid_backlink_cnt += note_valid_backlink_cnt;
            invalid_backlink_cnt += note_invalid_backlink_cnt;
        }

        Self {
            valid_backlink_cnt,
            invalid_backlink_cnt,
            source_and_link_to_target_list,
        }
    }

    pub fn get_valid_backlink_cnt(&self) -> usize {
        self.valid_backlink_cnt
    }

    pub fn get_invalid_backlink_cnt(&self) -> usize {
        self.invalid_backlink_cnt
    }
}

impl AllNoteLinksIterable for NoteLinkIndex {
    type Iter = std::vec::IntoIter<LinkFromSourceToTarget>;
    fn all_iter(&self) -> Self::Iter {
        self.source_and_link_to_target_list.clone().into_iter()
    }
}
