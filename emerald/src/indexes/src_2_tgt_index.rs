use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::content_analyzers::LinkSrc2TgtIterBoxed;
use crate::types::{LinkSrc2Tgt, ResourceId};
use crate::Result;

use super::src_2_tgt_iter_src::Src2TgtIterSrc;

#[derive(Clone)]
pub struct Src2TargetIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,
    src_2_tgt_list: Rc<Vec<LinkSrc2Tgt>>,
}

impl Src2TargetIndex {
    pub fn new<'a>(
        iter: impl Iterator<Item = (ResourceId, Result<LinkSrc2TgtIterBoxed<'a>>)>,
    ) -> Self {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut src_2_tgt_list = Vec::<LinkSrc2Tgt>::new();

        for (src, res_vec) in iter {
            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for s2t in res_vec.unwrap().into_iter() {
                match &s2t {
                    LinkSrc2Tgt {
                        src: _,
                        link,
                        tgt: None,
                    } => {
                        note_invalid_backlink_cnt += 1;
                        warn!("Invalid link '{:?}' found in '{:?}'", &link, &src);
                    }
                    _ => note_valid_backlink_cnt += 1,
                }
                src_2_tgt_list.push(s2t);
            }

            if note_valid_backlink_cnt == 0 {
                trace!("No valid links found in {:?}", &src);
            }

            valid_backlink_cnt += note_valid_backlink_cnt;
            invalid_backlink_cnt += note_invalid_backlink_cnt;
        }

        Self {
            valid_backlink_cnt,
            invalid_backlink_cnt,
            src_2_tgt_list: Rc::new(src_2_tgt_list),
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
        (*self.src_2_tgt_list).clone().into_iter()
    }
}
