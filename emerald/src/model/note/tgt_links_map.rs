use super::tgt_iter_retriever::TgtIterRetriever;
use crate::types;
use std::collections::{hash_map::Entry, HashMap};

type Src2Link2TgtMap = HashMap<types::ResourceId, Vec<types::Link2Tgt>>;

#[derive(Clone)]
pub struct TgtLinksMap {
    link_2_tgt_map: Src2Link2TgtMap,
}

impl TgtLinksMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = &'a types::LinkSrc2Tgt>) -> Self {
        let mut link_2_tgt_map = Src2Link2TgtMap::new();
        for s2t in it_src.into_iter() {
            let link_to_target = s2t.get_link_to_target();

            match link_2_tgt_map.entry(s2t.src.clone()) {
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
    fn retrieve(&self, src: &types::ResourceId) -> Option<std::vec::IntoIter<types::Link2Tgt>> {
        self.link_2_tgt_map.get(src).map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::TgtIterRetriever;
    use super::TgtLinksMap;
    use crate::types::Link2Tgt;
    use crate::types::LinkSrc2Tgt;

    #[test]
    fn test_one_match() {
        let test_data: Vec<LinkSrc2Tgt> = vec![("o1", "o1->d1", "d1").into()];
        let dut = TgtLinksMap::new(test_data.iter());
        let res: Vec<Link2Tgt> = dut.retrieve(&("o1".into())).unwrap().collect();

        assert_eq!(res, vec![Link2Tgt::new("o1->d1".into(), Some("d1".into()))]);
    }

    #[test]
    fn test_two_matches() {
        let test_data: Vec<LinkSrc2Tgt> =
            vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];

        let dut = TgtLinksMap::new(test_data.iter());
        let res: Vec<Link2Tgt> = dut.retrieve(&("o1".into())).unwrap().collect();

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
        let test_data: Vec<LinkSrc2Tgt> = vec![
            ("doesn't matter 1", "abc", "def").into(),
            ("o1", "o1->d1", "d1").into(),
            ("doesn't matter 2", "abc", "def").into(),
            ("o1", "o1->d2", "d2").into(),
            ("doesn't matter 3", "abc", "def").into(),
        ];

        let dut = TgtLinksMap::new(test_data.iter());
        let res: Vec<Link2Tgt> = dut.retrieve(&("o1".into())).unwrap().collect();

        assert_eq!(
            res,
            vec![
                Link2Tgt::new("o1->d1".into(), Some("d1".into())),
                Link2Tgt::new("o1->d2".into(), Some("d2".into()))
            ]
        );
    }
}
