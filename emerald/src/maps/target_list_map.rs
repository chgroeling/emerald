use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::AllNoteLinksIterable,
    types::{LinkToTarget, ResourceId},
};

use super::target_iterator_queryable::TargetIteratorQueryable;

pub type LinkToTargetList = Vec<LinkToTarget>;
type SourceToLinkToTargetList = HashMap<ResourceId, LinkToTargetList>;

pub struct TargetListMap {
    source_to_target_map: SourceToLinkToTargetList,
}

impl TargetListMap {
    pub fn new(note_links_iterable: &impl AllNoteLinksIterable) -> Self {
        let mut source_to_target_map = SourceToLinkToTargetList::new();
        for s2t in note_links_iterable.all_iter() {
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
    use super::AllNoteLinksIterable;
    use super::TargetIteratorQueryable;
    use super::TargetListMap;
    use crate::types::LinkFromSourceToTarget;
    use crate::types::LinkToTarget;

    struct NotesIterSource(Vec<LinkFromSourceToTarget>);
    impl AllNoteLinksIterable for NotesIterSource {
        type Iter = std::vec::IntoIter<LinkFromSourceToTarget>;

        fn all_iter(&self) -> Self::Iter {
            self.0.clone().into_iter()
        }
    }

    /// Create a SourceAndLinkToTarget struct for test purposes
    fn sample_slt(src: &str, link: &str, target: &str) -> LinkFromSourceToTarget {
        LinkFromSourceToTarget::new(src.into(), link.into(), Some(target.into()))
    }
    #[test]
    fn test_one_match() {
        let data = NotesIterSource(vec![sample_slt("o1", "o1->d1", "d1")]);

        let dut = TargetListMap::new(&data);
        let res: Vec<LinkToTarget> = dut.query("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![LinkToTarget::new("o1->d1".into(), Some("d1".into()))]
        );
    }

    #[test]
    fn test_two_matches() {
        let data = NotesIterSource(vec![
            sample_slt("o1", "o1->d1", "d1"),
            sample_slt("o1", "o1->d2", "d2"),
        ]);

        let dut = TargetListMap::new(&data);
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
        let data = NotesIterSource(vec![
            sample_slt("doesn't matter 1", "abc", "def"),
            sample_slt("o1", "o1->d1", "d1"),
            sample_slt("doesn't matter 2", "abc", "def"),
            sample_slt("o1", "o1->d2", "d2"),
            sample_slt("doesn't matter 3", "abc", "def"),
        ]);

        let dut = TargetListMap::new(&data);
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
