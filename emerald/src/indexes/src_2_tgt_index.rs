use std::rc::Rc;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use crate::{
    resources::content_loader::ContentLoader,
    types::{Content, Link2Tgt, LinkSrc2Tgt},
};

use super::{src_2_tgt_iter_src::Src2TgtIterSrc, ResourceIdsIterSrc};

#[derive(Clone)]
pub struct Src2TargetIndex {
    valid_backlink_cnt: usize,
    invalid_backlink_cnt: usize,
    src_2_tgt_list: Rc<Vec<LinkSrc2Tgt>>,
}

impl Src2TargetIndex {
    pub fn new<U, F>(
        content_loader: &impl ContentLoader,
        md_resource_ids_iter_rc: &impl ResourceIdsIterSrc,
        extract_links_from_md: F,
    ) -> Self
    where
        F: Fn(Content) -> U,
        U: Iterator<Item = Link2Tgt>,
    {
        let mut valid_backlink_cnt: usize = 0;
        let mut invalid_backlink_cnt: usize = 0;
        let mut src_2_tgt_list = Vec::<LinkSrc2Tgt>::new();

        for src in md_resource_ids_iter_rc.iter() {
            let content = content_loader.load(&src).unwrap();
            trace!("Link extraction from {:?} starts", &src);

            let mut note_valid_backlink_cnt: usize = 0;
            let mut note_invalid_backlink_cnt: usize = 0;
            for link_to_target in extract_links_from_md(content) {
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
