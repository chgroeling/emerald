use crate::types;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::rc::Rc;

#[derive(Clone)]
pub struct Src2TargetIndex {
    src_2_tgt_list: Rc<Vec<types::LinkSrc2Tgt>>,
}

impl Src2TargetIndex {
    pub fn new(it_src: impl IntoIterator<Item = types::LinkSrc2Tgt>) -> Self {
        Self {
            src_2_tgt_list: Rc::new(it_src.into_iter().collect()),
        }
    }
}

impl<'a> IntoIterator for &'a Src2TargetIndex {
    type Item = &'a types::LinkSrc2Tgt;
    type IntoIter = std::slice::Iter<'a, types::LinkSrc2Tgt>;

    fn into_iter(self) -> Self::IntoIter {
        self.src_2_tgt_list.iter()
    }
}
