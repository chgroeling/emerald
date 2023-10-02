use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::Src2TgtIterSrc,
    types::{Link2Tgt, ResourceId},
};

use super::tgt_iter_retriever::TgtIterRetriever;

type Src2Link2TgtMap = HashMap<ResourceId, Vec<Link2Tgt>>;

pub struct TgtLinksMap {
    link_2_tgt_map: Src2Link2TgtMap,
}

impl TgtLinksMap {
    pub fn new(link_s2t_iter_rc: &impl Src2TgtIterSrc) -> Self {
        let mut link_2_tgt_map = Src2Link2TgtMap::new();
        for s2t in link_s2t_iter_rc.iter() {
            let link_to_target = s2t.get_link_to_target();

            match link_2_tgt_map.entry(s2t.src) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_to_target);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_to_target]);
                }
            }
        }
        Self { link_2_tgt_map }
    }
}

impl TgtIterRetriever for TgtLinksMap {
    fn query(&self, src: ResourceId) -> Option<std::vec::IntoIter<Link2Tgt>> {
        self.link_2_tgt_map.get(&src).map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::TgtIterRetriever;
    use super::TgtLinksMap;
    use crate::indexes::src_2_tgt_iter_src::MockSrc2TgtIterSrc;
    use crate::types::Link2Tgt;

    #[test]
    fn test_one_match() {
        let test_data = vec![("o1", "o1->d1", "d1").into()];
        let mut mock = MockSrc2TgtIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = TgtLinksMap::new(&mock);
        let res: Vec<Link2Tgt> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(res, vec![Link2Tgt::new("o1->d1".into(), Some("d1".into()))]);
    }

    #[test]
    fn test_two_matches() {
        let test_data = vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];
        let mut mock = MockSrc2TgtIterSrc::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = TgtLinksMap::new(&mock);
        let res: Vec<Link2Tgt> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![
                Link2Tgt::new("o1->d1".into(), Some("d1".into())),
                Link2Tgt::new("o1->d2".into(), Some("d2".into()))
            ]
        );
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

        let dut = TgtLinksMap::new(&mock);
        let res: Vec<Link2Tgt> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![
                Link2Tgt::new("o1->d1".into(), Some("d1".into())),
                Link2Tgt::new("o1->d2".into(), Some("d2".into()))
            ]
        );
    }
}
