use std::collections::HashMap;

use crate::{indexes::AllNoteLinksIterSource, types::ResourceId};

use super::destination_list_resolver::LinkWithDestinationList;

type OriginToDestinationListMap = HashMap<ResourceId, LinkWithDestinationList>;
struct DestinationListMap {
    origin_to_destination: OriginToDestinationListMap,
}

impl DestinationListMap {
    pub fn new(all_note_links_iter_source: &impl AllNoteLinksIterSource) -> Self {
        let origin_to_destination = OriginToDestinationListMap::new();
        for link_to_dest in all_note_links_iter_source.all_iter() {

            //
        }
        Self {
            origin_to_destination,
        }
    }
}
