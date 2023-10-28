use crate::types;

use super::src_links_map::SrcLinksMap;
use super::tgt_links_map::TgtLinksMap;

pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    tgt_links_map: TgtLinksMap,
    src_links_map: SrcLinksMap,
}

impl DefaultNoteModel {
    pub fn new<'a>(
        it_notes: impl IntoIterator<Item = &'a types::ResourceId>,
        it_note_meta_data: impl IntoIterator<Item = &'a (&'a types::ResourceId, types::MetaData)>,
        it_links_src_2_tgt: impl IntoIterator<Item = &'a types::LinkSrc2Tgt> + Clone,
    ) -> DefaultNoteModel {
        DefaultNoteModel {
            note_index: it_notes.into_iter().cloned().collect(),
            tgt_links_map: TgtLinksMap::new(it_links_src_2_tgt.clone()),
            src_links_map: SrcLinksMap::new(it_links_src_2_tgt),
        }
    }
}

impl<'a> IntoIterator for &'a DefaultNoteModel {
    type Item = &'a types::ResourceId;

    type IntoIter = std::slice::Iter<'a, types::ResourceId>;

    fn into_iter(self) -> Self::IntoIter {
        let note_idx = &self.note_index;
        note_idx.into_iter()
    }
}
