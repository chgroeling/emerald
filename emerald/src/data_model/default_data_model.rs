use crate::{
    maps,
    types::{self, Link},
};

pub struct DefaultDataModel {
    note_index: Vec<types::ResourceId>,
    file_index: Vec<types::ResourceId>,
    tgt_links_map: maps::TgtLinksMap,
    src_links_map: maps::SrcLinksMap,
}

impl DefaultDataModel {
    pub fn new<'a>(
        it_notes: impl IntoIterator<Item = &'a types::ResourceId>,
        it_files: impl IntoIterator<Item = &'a types::ResourceId>,
        it_links_src_2_tgt: impl IntoIterator<Item = &'a types::LinkSrc2Tgt> + Clone,
    ) -> DefaultDataModel {
        DefaultDataModel {
            note_index: it_notes.into_iter().cloned().collect(),
            file_index: it_files.into_iter().cloned().collect(),
            tgt_links_map: maps::TgtLinksMap::new(it_links_src_2_tgt.clone()),
            src_links_map: maps::SrcLinksMap::new(it_links_src_2_tgt),
        }
    }
}
