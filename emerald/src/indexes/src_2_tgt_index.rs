#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    content_analyzers::MdLinkAnalyzerIterSrc,
    resources::ContentIterSrc,
    types::{Link2Tgt, LinkSrc2Tgt},
};

use super::src_2_tgt_iter_src::Src2TgtIterSrc;

pub struct Src2TargetIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,
    src_2_tgt_list: Vec<LinkSrc2Tgt>,
}

impl Src2TargetIndex {
    pub fn new(
        content_iter_rc: &impl ContentIterSrc,
        md_link_analyer_iter_rc: &impl MdLinkAnalyzerIterSrc,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut src_2_tgt_list = Vec::<LinkSrc2Tgt>::new();

        for (src, content) in content_iter_rc.iter() {
            trace!("Link extraction from {:?} starts", &src);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_to_target in md_link_analyer_iter_rc.create_iter(content.0.as_ref().clone()) {
                match &link_to_target {
                    Link2Tgt { link, tgt: None } => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Parsing {:?} -> Link not found: {:?}", &src, &link);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                let s2t = LinkSrc2Tgt::from_link_to_target(src.clone(), link_to_target);
                src_2_tgt_list.push(s2t);
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
            src_2_tgt_list,
        }
    }

    pub fn get_valid_backlink_cnt(&self) -> usize {
        self.valid_backlink_cnt
    }

    pub fn get_invalid_backlink_cnt(&self) -> usize {
        self.invalid_backlink_cnt
    }
}

impl Src2TgtIterSrc for Src2TargetIndex {
    type Iter = std::vec::IntoIter<LinkSrc2Tgt>;
    fn iter(&self) -> Self::Iter {
        self.src_2_tgt_list.clone().into_iter()
    }
}
