use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::types::{LinkSrc2Tgt, ResourceId};

#[derive(Clone)]
pub struct Src2TargetIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,
    src_2_tgt_list: Rc<Vec<LinkSrc2Tgt>>,
}

impl Src2TargetIndex {
    pub fn new(iter: impl Iterator<Item = LinkSrc2Tgt>) -> Self {
        let mut src_2_tgt_list = Vec::<LinkSrc2Tgt>::new();

        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut note_valid_backlink_cnt: usize = 0;
        let mut note_invalid_backlink_cnt: usize = 0;

        let mut iter_mut = iter.peekable();
        let mut opt_last_src: Option<ResourceId> = None;
        loop {
            let Some(s2t) = iter_mut.next() else {
                if let Some(last_src) = opt_last_src {
                    if note_valid_backlink_cnt == 0 {
                        trace!("No valid links found in {:?}", &last_src);
                    }
                }
                valid_backlink_cnt += note_valid_backlink_cnt;
                invalid_backlink_cnt += note_invalid_backlink_cnt;

                break;
            };

            match &s2t {
                LinkSrc2Tgt {
                    src,
                    link,
                    tgt: None,
                } => {
                    note_invalid_backlink_cnt += 1;
                    warn!("Invalid link '{:?}' found in '{:?}'", &link, &src);
                }
                _ => note_valid_backlink_cnt += 1,
            }

            opt_last_src = Some(s2t.src.clone());

            // Check if next element has a different source
            if let Some(next_s2t) = iter_mut.peek() {
                if next_s2t.src != s2t.src {
                    if note_valid_backlink_cnt == 0 {
                        trace!("No valid links found in {:?}", &s2t.src);
                    }
                    valid_backlink_cnt += note_valid_backlink_cnt;
                    invalid_backlink_cnt += note_invalid_backlink_cnt;
                    note_valid_backlink_cnt = 0;
                    note_invalid_backlink_cnt = 0;
                }
            }
            src_2_tgt_list.push(s2t);
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
    pub fn iter(&self) -> impl Iterator<Item = &'_ LinkSrc2Tgt> {
        self.src_2_tgt_list.iter()
    }
}
