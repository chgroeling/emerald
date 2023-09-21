#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSource,
    resources::ContentIterSource,
    types::{LinkToTarget, SourceAndLinkToTarget},
};

use super::all_note_links_iter_source::AllNoteLinksIterSource;

#[allow(dead_code)]
pub type SourceAndLinkToTargetList = Vec<SourceAndLinkToTarget>;

pub struct NoteLinkIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,

    #[allow(dead_code)]
    source_and_link_to_target_list: SourceAndLinkToTargetList,
}

impl NoteLinkIndex {
    pub fn new(
        content_iter_src: &impl ContentIterSource,
        md_link_analyzer: &impl MdLinkAnalyzerIterSource,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut source_and_link_to_target_list = SourceAndLinkToTargetList::new();

        for (source, content) in content_iter_src.iter() {
            trace!("Link extraction from {:?} starts", &source);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_to_target in md_link_analyzer.create_iter(content.0.as_ref().clone()) {
                match &link_to_target {
                    LinkToTarget { link, target: None } => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &source, &link);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                let note_link = SourceAndLinkToTarget::new(source.clone(), link_to_target);
                source_and_link_to_target_list.push(note_link);
            }

            if note_valid_backlink_cnt == 0 {
                trace!("No valid links found in  {:?}", &source);
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

impl AllNoteLinksIterSource for NoteLinkIndex {
    type Iter = std::vec::IntoIter<SourceAndLinkToTarget>;
    fn all_iter(&self) -> Self::Iter {
        self.source_and_link_to_target_list.clone().into_iter()
    }
}
