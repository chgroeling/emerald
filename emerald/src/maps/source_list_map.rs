use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::SrcTgtIterable,
    types::{LinkFromSource, ResourceId},
};

use super::src_iter_queryable::SrcIterQueryable;

pub type LinkFromSourceList = Vec<LinkFromSource>;
type TargetToLinkFromSourceList = HashMap<ResourceId, LinkFromSourceList>;

pub struct SourceListMap {
    source_to_target_map: TargetToLinkFromSourceList,
}

impl SourceListMap {
    pub fn new(link_s2t_iterable: &impl SrcTgtIterable) -> Self {
        let mut target_to_source_map = TargetToLinkFromSourceList::new();
        for s2t in link_s2t_iterable.iter() {
            let link_from_source = s2t.get_link_from_source();
            let target = if let Some(target) = s2t.target {
                target
            } else {
                continue;
            };
            match target_to_source_map.entry(target) {
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

impl SrcIterQueryable for SourceListMap {
    fn query(&self, source: ResourceId) -> Option<std::vec::IntoIter<LinkFromSource>> {
        self.source_to_target_map
            .get(&source)
            .map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkFromSource;
    use super::SourceListMap;
    use super::SrcIterQueryable;
    use crate::indexes::src_tgt_iterable::MockSrcTgtIterable;

    #[test]
    fn test_one_match() {
        let test_data = vec![("o1", "o1->d1", "d1").into()];

        let mut mock = MockSrcTgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SourceListMap::new(&mock);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFromSource::new("o1->d1".into(), "o1".into())]);
    }

    #[test]
    fn test_two_matches() {
        let test_data = vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];

        let mut mock = MockSrcTgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = SourceListMap::new(&mock);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFromSource::new("o1->d1".into(), "o1".into())]);
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

        let mut mock = MockSrcTgtIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());
        let dut = SourceListMap::new(&mock);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![LinkFromSource::new("o1->d1".into(), "o1".into(),)]
        );
    }
}
