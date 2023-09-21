use std::collections::{hash_map::Entry, HashMap};

use crate::{indexes::AllNoteLinksIterSource, types::ResourceId};

use super::destination_list_resolver::{DestinationListResolver, ListOfLinksWithDestination};

type OriginToDestinationListMap = HashMap<ResourceId, ListOfLinksWithDestination>;
struct DestinationListMap {
    origin_to_destination: OriginToDestinationListMap,
}

impl DestinationListMap {
    pub fn new(all_note_links_iter_source: &impl AllNoteLinksIterSource) -> Self {
        let mut origin_to_destination = OriginToDestinationListMap::new();
        for link_to_dest in all_note_links_iter_source.all_iter() {
            let origin = link_to_dest.origin;
            let link_and_dest = link_to_dest.link_and_destination;

            match origin_to_destination.entry(origin) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(link_and_dest);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![link_and_dest]);
                }
            }
        }
        Self {
            origin_to_destination,
        }
    }
}

impl DestinationListResolver for DestinationListMap {
    fn resolve(&self, resource_id: ResourceId) -> std::vec::IntoIter<ListOfLinksWithDestination> {
        todo!()
    }
}
