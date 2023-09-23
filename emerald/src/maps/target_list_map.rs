use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::LinkFromSourceToTargetIterable,
    types::{LinkToTarget, ResourceId},
};

use super::target_iterator_queryable::TargetIteratorQueryable;

pub type LinkToTargetList = Vec<LinkToTarget>;
type SourceToLinkToTargetList = HashMap<ResourceId, LinkToTargetList>;

pub struct TargetListMap {
    source_to_target_map: SourceToLinkToTargetList,
}

impl TargetListMap {
    pub fn new(link_s2t_iterable: &impl LinkFromSourceToTargetIterable) -> Self {
        let mut source_to_target_map = SourceToLinkToTargetList::new();
        for s2t in link_s2t_iterable.iter() {
            let link_to_target = s2t.get_link_to_target();

            match source_to_target_map.entry(s2t.source) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_to_target);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_to_target]);
                }
            }
        }
        Self {
            source_to_target_map,
        }
    }
}

impl TargetIteratorQueryable for TargetListMap {
    fn query(&self, source: ResourceId) -> Option<std::vec::IntoIter<LinkToTarget>> {
        self.source_to_target_map
            .get(&source)
            .map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::TargetIteratorQueryable;
    use super::TargetListMap;
    use crate::indexes::link_from_source_to_target_iterable::MockLinkFromSourceToTargetIterable;
    use crate::types::LinkToTarget;

    #[test]
    fn test_one_match() {
        let test_data = vec![("o1", "o1->d1", "d1").into()];
        let mut mock = MockLinkFromSourceToTargetIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = TargetListMap::new(&mock);
        let res: Vec<LinkToTarget> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![LinkToTarget::new("o1->d1".into(), Some("d1".into()))]
        );
    }

    #[test]
    fn test_two_matches() {
        let test_data = vec![("o1", "o1->d1", "d1").into(), ("o1", "o1->d2", "d2").into()];
        let mut mock = MockLinkFromSourceToTargetIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = TargetListMap::new(&mock);
        let res: Vec<LinkToTarget> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![
                LinkToTarget::new("o1->d1".into(), Some("d1".into())),
                LinkToTarget::new("o1->d2".into(), Some("d2".into()))
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
        let mut mock = MockLinkFromSourceToTargetIterable::new();
        mock.expect_iter().return_const(test_data.into_iter());

        let dut = TargetListMap::new(&mock);
        let res: Vec<LinkToTarget> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![
                LinkToTarget::new("o1->d1".into(), Some("d1".into())),
                LinkToTarget::new("o1->d2".into(), Some("d2".into()))
            ]
        );
    }
}
