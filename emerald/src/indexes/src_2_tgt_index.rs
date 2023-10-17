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
    pub fn new(it_src: impl IntoIterator<Item = LinkSrc2Tgt>) -> Self {
        let mut src_2_tgt_list = Vec::<LinkSrc2Tgt>::new();

        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut note_valid_backlink_cnt: usize = 0;
        let mut note_invalid_backlink_cnt: usize = 0;

        let mut iter_mut = it_src.into_iter();
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

            // Check if this element has a different source than the one before
            if let Some(last_src) = opt_last_src {
                if last_src != s2t.src {
                    if note_valid_backlink_cnt == 0 {
                        trace!("No valid links found in {:?}", &last_src);
                    }
                    valid_backlink_cnt += note_valid_backlink_cnt;
                    invalid_backlink_cnt += note_invalid_backlink_cnt;
                    note_valid_backlink_cnt = 0;
                    note_invalid_backlink_cnt = 0;
                }
            }

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
}

impl<'a> IntoIterator for &'a Src2TargetIndex {
    type Item = &'a LinkSrc2Tgt;

    type IntoIter = std::slice::Iter<'a, LinkSrc2Tgt>;

    fn into_iter(self) -> Self::IntoIter {
        self.src_2_tgt_list.iter()
    }
}
