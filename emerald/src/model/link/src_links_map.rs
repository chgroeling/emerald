use super::src_iter_retriever::SrcIterRetriever;
use crate::types;
use std::collections::{hash_map::Entry, HashMap};

type Tgt2LinkFrmSrcMap = HashMap<types::ResourceId, Vec<types::LinkFrmSrc>>;

#[derive(Clone)]
pub struct SrcLinksMap {
    src_2_tgt_map: Tgt2LinkFrmSrcMap,
}

impl SrcLinksMap {
    pub fn new<'a>(it_src: impl IntoIterator<Item = &'a types::LinkSrc2Tgt>) -> Self {
        let mut src_2_tgt_map = Tgt2LinkFrmSrcMap::new();
        for s2t in it_src.into_iter() {
            let link_from_source = s2t.get_link_from_source();
            let tgt = if let Some(tgt) = s2t.tgt.clone() {
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
    fn retrieve(
        &self,
        tgt: &types::ResourceId,
    ) -> Option<Box<dyn Iterator<Item = types::LinkFrmSrc>>> {
        if let Some(vec) = self.src_2_tgt_map.get(tgt) {
            Some(Box::new(vec.clone().into_iter()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::types;
    use super::SrcIterRetriever;
    use super::SrcLinksMap;
    use crate::types::LinkSrc2Tgt;

    #[test]
    fn test_one_match() {
        let test_data: Vec<LinkSrc2Tgt> = vec![("o1", "o1->d1", "d1").into()];

        let dut = SrcLinksMap::new(test_data.iter());
        let res: Vec<types::LinkFrmSrc> = dut.retrieve(&("d1".into())).unwrap().collect();

        assert_eq!(
            res,
            vec![types::LinkFrmSrc::new("o1->d1".into(), "o1".into())]
        );
    }

    #[test]
    fn test_two_matches() {
        let test_data: Vec<LinkSrc2Tgt> =
            vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];

        let dut = SrcLinksMap::new(test_data.iter());
        let res: Vec<types::LinkFrmSrc> = dut.retrieve(&("d1".into())).unwrap().collect();

        assert_eq!(
            res,
            vec![types::LinkFrmSrc::new("o1->d1".into(), "o1".into())]
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

        let dut = SrcLinksMap::new(test_data.iter());
        let res: Vec<types::LinkFrmSrc> = dut.retrieve(&("d1".into())).unwrap().collect();

        assert_eq!(
            res,
            vec![types::LinkFrmSrc::new("o1->d1".into(), "o1".into(),)]
        );
    }
}
