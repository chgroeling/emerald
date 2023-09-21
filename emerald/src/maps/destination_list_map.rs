use std::collections::{hash_map::Entry, HashMap};

use crate::{
    indexes::AllNoteLinksIterSource,
    types::{LinkToTarget, ResourceId},
};

use super::destination_iterator_queryable::DestinationIteratorQueryable;

type OriginToDestinationListMap = HashMap<ResourceId, ListOfLinksWithDestination>;

pub type ListOfLinksWithDestination = Vec<LinkToTarget>;

pub struct DestinationListMap {
    origin_to_destination: OriginToDestinationListMap,
}

impl DestinationListMap {
    pub fn new(all_note_links_iter_source: &impl AllNoteLinksIterSource) -> Self {
        let mut origin_to_destination = OriginToDestinationListMap::new();
        for link_to_dest in all_note_links_iter_source.all_iter() {
            let source = link_to_dest.source;
            let link_to_target = link_to_dest.link_to_target;

            match origin_to_destination.entry(source) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_to_target);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_to_target]);
                }
            }
        }
        Self {
            origin_to_destination,
        }
    }
}

impl DestinationIteratorQueryable for DestinationListMap {
    fn query_destination_iter(
        &self,
        resource_id: ResourceId,
    ) -> Option<std::vec::IntoIter<LinkToTarget>> {
        self.origin_to_destination
            .get(&resource_id)
            .map(|f| f.clone().into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::AllNoteLinksIterSource;
    use super::DestinationIteratorQueryable;
    use super::DestinationListMap;
    use crate::types::LinkToTarget;
    use crate::types::SourceAndLinkToTarget;

    struct NotesIterSource(Vec<SourceAndLinkToTarget>);
    impl AllNoteLinksIterSource for NotesIterSource {
        type Iter = std::vec::IntoIter<SourceAndLinkToTarget>;

        fn all_iter(&self) -> Self::Iter {
            self.0.clone().into_iter()
        }
    }

    /// Create a OriginToDestination struct for test purposes
    fn sample_otd(src: &str, link: &str, dest: &str) -> SourceAndLinkToTarget {
        SourceAndLinkToTarget::new(
            src.into(),
            LinkToTarget::new(link.into(), Some(dest.into())),
        )
    }
    #[test]
    fn test_one_match() {
        let data = NotesIterSource(vec![sample_otd("o1", "o1->d1", "d1")]);

        let dut = DestinationListMap::new(&data);
        let res: Vec<LinkToTarget> = dut.query_destination_iter("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![LinkToTarget::new("o1->d1".into(), Some("d1".into()))]
        );
    }

    #[test]
    fn test_two_matches() {
        let data = NotesIterSource(vec![
            sample_otd("o1", "o1->d1", "d1"),
            sample_otd("o1", "o1->d2", "d2"),
        ]);

        let dut = DestinationListMap::new(&data);
        let res: Vec<LinkToTarget> = dut.query_destination_iter("o1".into()).unwrap().collect();

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
            sample_otd("doesn't matter 1", "abc", "def"),
            sample_otd("o1", "o1->d1", "d1"),
            sample_otd("doesn't matter 2", "abc", "def"),
            sample_otd("o1", "o1->d2", "d2"),
            sample_otd("doesn't matter 3", "abc", "def"),
        ]);

        let dut = DestinationListMap::new(&data);
        let res: Vec<LinkToTarget> = dut.query_destination_iter("o1".into()).unwrap().collect();

        assert_eq!(
            res,
            vec![
                LinkToTarget::new("o1->d1".into(), Some("d1".into())),
                LinkToTarget::new("o1->d2".into(), Some("d2".into()))
            ]
        );
    }
}
