use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::AllNoteLinksIterable,
    types::{LinkFromSource, ResourceId},
};

use super::source_iterator_queryable::SourceIteratorQueryable;

pub type LinkFromSourceList = Vec<LinkFromSource>;
type TargetToLinkFromSourceList = HashMap<ResourceId, LinkFromSourceList>;

pub struct SourceListMap {
    source_to_target_map: TargetToLinkFromSourceList,
}

impl SourceListMap {
    pub fn new(note_links_iterable: &impl AllNoteLinksIterable) -> Self {
        let mut target_to_source_map = TargetToLinkFromSourceList::new();
        for s2t in note_links_iterable.all_iter() {
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

impl SourceIteratorQueryable for SourceListMap {
    fn query(&self, source: ResourceId) -> Option<std::vec::IntoIter<LinkFromSource>> {
        self.source_to_target_map
            .get(&source)
            .map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::LinkFromSourceToTarget;

    use super::AllNoteLinksIterable;
    use super::LinkFromSource;
    use super::SourceIteratorQueryable;
    use super::SourceListMap;

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

        let dut = SourceListMap::new(&data);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFromSource::new("o1->d1".into(), "o1".into())]);
    }

    #[test]
    fn test_two_matches() {
        let data = NotesIterSource(vec![
            sample_slt("o1", "o1->d1", "d1"),
            sample_slt("o1", "o1->d2", "d2"),
        ]);

        let dut = SourceListMap::new(&data);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(res, vec![LinkFromSource::new("o1->d1".into(), "o1".into())]);
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

        let dut = SourceListMap::new(&data);
        let res: Vec<LinkFromSource> = dut.query("d1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![LinkFromSource::new("o1->d1".into(), "o1".into(),)]
        );
    }
}
