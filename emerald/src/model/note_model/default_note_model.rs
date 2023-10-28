use crate::types;

use super::meta_data_map::MetaDataMap;
use super::meta_data_retriever::MetaDataRetriever;
use super::src_links_map::SrcLinksMap;
use super::tgt_links_map::TgtLinksMap;

pub struct DefaultNoteModel {
    note_index: Vec<types::ResourceId>,
    tgt_links_map: TgtLinksMap,
    src_links_map: SrcLinksMap,
    meta_data_map: MetaDataMap,
}

impl DefaultNoteModel {
    pub fn new<'a>(
        it_note_meta_data: impl IntoIterator<Item = (&'a types::ResourceId, &'a types::MetaData)>
            + Clone,
        it_links_src_2_tgt: impl IntoIterator<Item = &'a types::LinkSrc2Tgt> + Clone,
    ) -> DefaultNoteModel {
        DefaultNoteModel {
            note_index: it_note_meta_data
                .clone()
                .into_iter()
                .map(|f| f.0)
                .cloned()
                .collect(),
            tgt_links_map: TgtLinksMap::new(it_links_src_2_tgt.clone()),
            src_links_map: SrcLinksMap::new(it_links_src_2_tgt),
            meta_data_map: MetaDataMap::new(it_note_meta_data),
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

impl MetaDataRetriever for DefaultNoteModel {
    fn retrieve(&self, md: types::ResourceId) -> &types::MetaData {
        // Option is not returned because meta data should be consistent at this point
        self.meta_data_map.retrieve(md)
    }
}
