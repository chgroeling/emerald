use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::Src2TgtIterSrc,
    types::{LinkFrmSrc, ResourceId},
};

use super::src_iter_retriever::SrcIterRetriever;

type Tgt2LinkFrmSrcMap = HashMap<ResourceId, Vec<LinkFrmSrc>>;

pub struct SrcLinksMap {
    src_2_tgt_map: Tgt2LinkFrmSrcMap,
}

impl SrcLinksMap {
    pub fn new(link_s2t_iter_rc: &impl Src2TgtIterSrc) -> Self {
        let mut src_2_tgt_map = Tgt2LinkFrmSrcMap::new();
        for s2t in link_s2t_iter_rc.iter() {
            let link_from_source = s2t.get_link_from_source();
            let tgt = if let Some(tgt) = s2t.tgt {
                tgt
            } else {
                continue;
            };
            match src_2_tgt_map.entry(tgt) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_from_source);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_from_source]);
                }
            }
        }
        Self { src_2_tgt_map }
    }
}

impl SrcIterRetriever for SrcLinksMap {
    fn retrieve(&self, tgt: ResourceId) -> Option<std::vec::IntoIter<LinkFrmSrc>> {
        self.src_2_tgt_map.get(&tgt).map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkFrmSrc;
    use super::SrcIterRetriever;
    use super::SrcLinksMap;
    use crate::indexes::src_2_tgt_iter_src::MockSrc2TgtIterSrc;

    #[test]
    fn test_one_match() {
        let test_data = vec![("o1", "o1->d1", "d1").into()];

        let mut mock = MockSrc2TgtIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.retrieve("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFrmSrc::new("o1->d1".into(), "o1".into())]);
    }

    #[test]
    fn test_two_matches() {
        let test_data = vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];

        let mut mock = MockSrc2TgtIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.retrieve("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFrmSrc::new("o1->d1".into(), "o1".into())]);
    }

    #[test]
    fn test_two_matches_elements_inbetween() {
        let test_data = vec![
            ("doesn't matter 1", "abc", "def").into(),
            ("o1", "o1->d1", "d1").into(),
            ("doesn't matter 2", "abc", "def").into(),
            ("o1", "o1->d2", "d2").into(),
            ("doesn't matter 3", "abc", "def").into(),
        ];

        let mut mock = MockSrc2TgtIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());
        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.retrieve("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFrmSrc::new("o1->d1".into(), "o1".into(),)]);
    }
}
