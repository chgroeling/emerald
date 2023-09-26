use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::Src2TgtIterable,
    types::{LinkFrmSrc, ResourceId},
};

use super::src_iter_queryable::SrcIterQueryable;

pub type LinkFrmSrcList = Vec<LinkFrmSrc>;
type TargetToLinkFrmSrcList = HashMap<ResourceId, LinkFrmSrcList>;

pub struct SrcLinksMap {
    source_to_target_map: TargetToLinkFrmSrcList,
}

impl SrcLinksMap {
    pub fn new(link_s2t_iterable: &impl Src2TgtIterable) -> Self {
        let mut target_to_source_map = TargetToLinkFrmSrcList::new();
        for s2t in link_s2t_iterable.iter() {
            let link_from_source = s2t.get_link_from_source();
            let tgt = if let Some(target) = s2t.target {
                target
            } else {
                continue;
            };
            match target_to_source_map.entry(tgt) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_from_source);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_from_source]);
                }
            }
        }
        Self {
            source_to_target_map: target_to_source_map,
        }
    }
}

impl SrcIterQueryable for SrcLinksMap {
    fn query(&self, source: ResourceId) -> Option<std::vec::IntoIter<LinkFrmSrc>> {
        self.source_to_target_map
            .get(&source)
            .map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkFrmSrc;
    use super::SrcIterQueryable;
    use super::SrcLinksMap;
    use crate::indexes::src_2_tgt_iterable::MockSrc2TgtIterable;

    #[test]
    fn test_one_match() {
        let test_data = vec![("o1", "o1->d1", "d1").into()];

        let mut mock = MockSrc2TgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFrmSrc::new("o1->d1".into(), "o1".into())]);
    }

    #[test]
    fn test_two_matches() {
        let test_data = vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];

        let mut mock = MockSrc2TgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.query("d1".into()).unwrap().collect();

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

        let mut mock = MockSrc2TgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());
        let dut = SrcLinksMap::new(&mock);
        let res: Vec<LinkFrmSrc> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFrmSrc::new("o1->d1".into(), "o1".into(),)]);
    }
}
