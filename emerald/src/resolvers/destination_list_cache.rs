use std::collections::HashMap;

use crate::{indexes::AllNoteLinksIterSource, types::ResourceId};

use super::destination_list_resolver::LinkWithDestinationList;

struct DestinationListCache {
    origin_to_destination: HashMap<ResourceId, LinkWithDestinationList>,
}

impl DestinationListCache {
    pub fn new(all_note_links_iter_source: &impl AllNoteLinksIterSource) -> Self {
        for link_to_dest in all_note_links_iter_source.all_iter() {}
        Self {
            origin_to_destination: todo!(),
        }
    }
}
